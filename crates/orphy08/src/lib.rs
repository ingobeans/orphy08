mod functions;
mod shared;
mod types;

pub use shared::*;

use std::{
    io::{BufRead, BufReader, Read, Write, stdout},
    process::{self, Stdio},
    sync::mpsc::{Receiver, channel},
};

pub fn launch_orphy08() -> Receiver<OrphyMessage> {
    let (tx, rx) = channel();

    let exe_path = std::env::current_exe()
        .expect("ERROR: The path of the orphy08 executable couldn't be found");
    let parent = exe_path.parent().unwrap();

    let pico8_path = "./pico8_dyn";

    let mut child = process::Command::new(pico8_path)
        .env("LD_LIBRARY_PATH", parent)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
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
            reader.read_until(b'\n', &mut buf).unwrap();
            if buf.starts_with(STDOUT_MESSAGE_HEADER) {
                // get message type
                let message = OrphyMessage::from_index(buf[STDOUT_MESSAGE_HEADER.len()]);
                tx.send(message).unwrap();
            } else {
                stdout().write_all(b"LOG: ").unwrap();
                stdout().write_all(&buf).unwrap();
            }
        }
    });
    rx
}
