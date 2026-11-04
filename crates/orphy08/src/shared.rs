use std::{ffi::CStr, os::raw::c_void, ptr::null_mut, sync::Mutex};

use libc::{
    __errno_location, MAP_SHARED, O_CREAT, O_EXCL, O_RDWR, PROT_READ, PROT_WRITE, S_IRUSR, S_IWUSR,
    atexit, ftruncate, mmap, shm_open, shm_unlink, strerror,
};

pub const VIDEO_DATA_SIZE: usize = 128 * 128 * 4;
// pub const STDOUT_MESSAGE_HEADER: &str = "[orphy08]";
pub const VIDEO_MEMORY_LABEL: &CStr = c"/orphy08-video-memory";

pub enum OrphyMessage {
    NewFrame,
}
impl OrphyMessage {
    fn into_message(self) {
        match self {
            OrphyMessage::NewFrame => {}
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

extern "C" fn clean_up_memory_at_exit() {
    let items = OPENED_MEMORY_REGIONS.lock().unwrap();
    for item in items.iter() {
        println!("closing: {item:?}");
        unsafe { shm_unlink(item.as_ptr()) };
    }
}

pub fn access_shared_memory(name: &'static CStr, size: usize, create: bool) -> (i32, *mut c_void) {
    unsafe {
        atexit(clean_up_memory_at_exit);
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
