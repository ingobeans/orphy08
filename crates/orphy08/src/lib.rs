mod functions;
mod shared;
mod types;

pub use shared::*;

use std::{
    io::Read,
    process::{self, Stdio},
    sync::mpsc::{Receiver, Sender, channel},
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

    // Create a channel wrapper around child stdout.
    // Regular stdout can only be read blocking.
    std::thread::spawn(|| {
        // let mut buf = Vec::with_capacity(SCREEN_DATA_SIZE);
    });
    rx
}
