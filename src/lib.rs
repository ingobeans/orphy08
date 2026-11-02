#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_CreateWindow() -> usize {
    println!("hiii this is test !!");
    0
}
