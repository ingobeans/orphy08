# orphy08 <img width="32" alt="image" src="https://github.com/user-attachments/assets/893e056f-2322-4191-b0cf-b5eaa93a0204" />


<sup>AI-free codebase</sup>

> [!NOTE]  
> This project does not distribute Pico-8 binaries and you need an official Pico-8 license to use this.

orphy08 is a libretro core wrapper for official Pico-8 binaries.

A libretro core is the crossplatform standard which is used by emulator frontends, which means that most emulator frontends should be able to run orphy08. 

There does exist a few other Pico-8 emulation cores, though these usually implement the entire Pico-8 system from scratch, which in practice leads to poor game compatibility. By directly running the real official Pico-8 binaries, you still keep full compatibility and native performance, but which now can be ran in any emulator frontend.

The goal of the project is to run Pico-8 on my Anbernic RG SP. The firmware I have on it doesn't have a desktop environment so running Pico-8 normally naturally wouldn't work. By writing a libretro core I should be able to get it running natively!

This is actually my second time writing a Pico-8 emulation core, the first one worked in a fundementally different way and had poor performance.

## Development

The project is written in Rust, so make sure you have that installed.

To compile and run, you'll first need to provide your own Pico-8 files. Move `pico8_dyn` and `pico8.dat` into the root of this directory.

You should then be able to run:
```bash
cargo build && cp target/debug/liborphy08.so target/debug/libSDL2-2.0.so.0 && cargo r
```
