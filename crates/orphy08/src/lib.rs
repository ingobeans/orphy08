pub mod debug;
mod functions;
mod shared;
mod types;

pub use shared::*;

use std::{
    io::{BufRead, BufReader, Write, stdout},
    path::{Path, PathBuf},
    process::{self, Stdio},
    sync::mpsc::{Receiver, channel},
};

pub fn launch_orphy08(cart: &str, buffered: bool) -> Receiver<OrphyMessage> {
    let (tx, rx) = channel();

    let exe_path = std::env::current_exe()
        .expect("ERROR: The path of the orphy08 executable couldn't be found");
    let parent = exe_path.parent().unwrap();

    let pico8_path = find_pico8_binary(parent);

    let mut command = process::Command::new(pico8_path);
    command
        .stdout(Stdio::piped())
        .env("LD_LIBRARY_PATH", parent)
        .arg("-run")
        .arg(cart);

    if buffered {
        command.env(BUFFERED_ENV_KEY, "true");
    }

    let mut child = command.spawn().unwrap();

    let child_stdout = child
        .stdout
        .take()
        .expect("Couldn't pipe output of pico8 executable!");

    let mut reader = BufReader::new(child_stdout);

    // Create a channel wrapper around child stdout.
    // Regular stdout can only be read blocking.
    std::thread::spawn(move || {
        loop {
            let mut buf = Vec::with_capacity(128);
            let len = reader.read_until(b'\n', &mut buf).unwrap();
            if buf.starts_with(STDOUT_MESSAGE_HEADER) {
                // get message type
                let message = OrphyMessage::from_index(buf[STDOUT_MESSAGE_HEADER.len()]);
                let Ok(_) = tx.send(message) else {
                    break;
                };
            } else if len > 0 {
                stdout().write_all(b"LOG: ").unwrap();
                stdout().write_all(&buf).unwrap();
            } else {
                break;
            }
        }
        let _ = child.kill();
        child.wait().unwrap();
    });
    rx
}

fn find_pico8_binary(parent: &Path) -> PathBuf {
    let mut pico8_path = PathBuf::from("./pico8_dyn");
    if !pico8_path.exists() {
        pico8_path = parent.join("pico8_dyn");
        if !pico8_path.exists() {
            panic!(
                "ERROR: Couldn't find 'pico8_dyn'.
Looked in: 
    * Directory of current executable ({parent:?})
    * Current working directory ({:?})\n",
                std::env::current_dir().unwrap()
            )
        }
    }
    pico8_path
}
