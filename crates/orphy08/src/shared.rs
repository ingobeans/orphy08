use std::{ffi::CStr, os::raw::c_void, process::exit, ptr::null_mut, sync::Mutex};

use libc::{
    __errno_location, MAP_SHARED, O_CREAT, O_EXCL, O_RDWR, PROT_READ, PROT_WRITE, S_IRUSR, S_IWUSR,
    SIGINT, SIGSEGV, atexit, ftruncate, mmap, shm_open, shm_unlink, signal, strerror, strsignal,
};

pub const VIDEO_DATA_SIZE: usize = 128 * 128 * 4;
pub const STDOUT_MESSAGE_HEADER: &[u8] = b"[orphy08]";
pub const BUFFERED_ENV_KEY: &str = "orphy08-video-buffered";

pub const VIDEO_MEMORY_LABEL: &CStr = c"/orphy08-video-memory";

pub enum OrphyMessage {
    NewFrame,
}
impl OrphyMessage {
    pub fn to_index(self) -> u8 {
        match self {
            OrphyMessage::NewFrame => 0,
        }
    }
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => OrphyMessage::NewFrame,
            other => {
                panic!("ERROR: {other} is not a valid OrphyMessage index!")
            }
        }
    }
}

fn check_error(res: i32, message: &str) {
    unsafe {
        if res == -1 {
            let err = __errno_location();
            let text = strerror(*err);
            panic!("ERROR: {message}. Err: {:?}", CStr::from_ptr(text));
        }
    }
}

static OPENED_MEMORY_REGIONS: Mutex<Vec<&'static CStr>> = Mutex::new(Vec::new());

extern "C" fn clean_up_memory_at_crash(value: i32) {
    let a = unsafe { CStr::from_ptr(strsignal(value)) };
    println!("\nProgram quit with signal: {a:?} ({value})");
    exit(1);
}

extern "C" fn clean_up_memory_at_exit() {
    let items = OPENED_MEMORY_REGIONS.lock().unwrap();
    for item in items.iter() {
        println!("Closing shared memory: {item:?}");
        unsafe { shm_unlink(item.as_ptr()) };
    }
}

pub fn set_up_memory_cleanup() {
    unsafe {
        atexit(clean_up_memory_at_exit);
        signal(SIGINT, clean_up_memory_at_crash as *const () as usize);
        signal(SIGSEGV, clean_up_memory_at_crash as *const () as usize);
    }
}

pub fn access_shared_memory(name: &'static CStr, size: usize, create: bool) -> (i32, *mut c_void) {
    unsafe {
        let mut flags = O_RDWR | O_CREAT;
        if create {
            flags |= O_EXCL;
        }
        let fd = shm_open(name.as_ptr() as _, flags, (S_IRUSR | S_IWUSR) as _);
        check_error(fd, "Couldn't open shared memory");

        OPENED_MEMORY_REGIONS.lock().unwrap().push(name);

        if create {
            let res = ftruncate(fd, size as _);
            check_error(res, "Couldn't truncate shared memory");
        }
        let addr = mmap(null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
        (fd, addr)
    }
}
