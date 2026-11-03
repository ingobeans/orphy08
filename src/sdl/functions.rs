// all the SDL2 functions that will need to be implemented

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_DestroyWindow() {println!("SDL_DestroyWindow")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_CreateRenderer() {println!("SDL_CreateRenderer")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_CreateWindow() {println!("SDL_CreateWindow")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetNumVideoDisplays() {println!("SDL_GetNumVideoDisplays")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_RenderPresent() {println!("SDL_RenderPresent")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GL_DeleteContext() {println!("SDL_GL_DeleteContext")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetCurrentAudioDriver() {println!("SDL_GetCurrentAudioDriver")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_LockAudio() {println!("SDL_LockAudio")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetWindowID() {println!("SDL_GetWindowID")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetNumAudioDrivers() {println!("SDL_GetNumAudioDrivers")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_UnlockAudio() {println!("SDL_UnlockAudio")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetDisplayBounds() {println!("SDL_GetDisplayBounds")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GL_CreateContext() {println!("SDL_GL_CreateContext")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetClipboardText() {println!("SDL_GetClipboardText")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetWindowSize() {println!("SDL_GetWindowSize")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickInstanceID() {println!("SDL_JoystickInstanceID")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetVersion() {println!("SDL_GetVersion")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GameControllerGetAttached() {println!("SDL_GameControllerGetAttached")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_DestroyRenderer() {println!("SDL_DestroyRenderer")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickClose() {println!("SDL_JoystickClose")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickOpen() {println!("SDL_JoystickOpen")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickNumAxes() {println!("SDL_JoystickNumAxes")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GameControllerGetAxis() {println!("SDL_GameControllerGetAxis")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetDisplayName() {println!("SDL_GetDisplayName")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetScancodeName() {println!("SDL_GetScancodeName")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_free() {println!("SDL_free")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_SetWindowGrab() {println!("SDL_SetWindowGrab")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetCurrentDisplayMode() {println!("SDL_GetCurrentDisplayMode")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetCurrentVideoDriver() {println!("SDL_GetCurrentVideoDriver")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetNumVideoDrivers() {println!("SDL_GetNumVideoDrivers")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_PollEvent() {println!("SDL_PollEvent")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetKeyboardFocus() {println!("SDL_GetKeyboardFocus")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_NumJoysticks() {println!("SDL_NumJoysticks")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GameControllerAddMapping() {println!("SDL_GameControllerAddMapping")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_DestroyTexture() {println!("SDL_DestroyTexture")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_Delay() {println!("SDL_Delay")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetError() {println!("SDL_GetError")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_UpdateTexture() {println!("SDL_UpdateTexture")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_WarpMouseInWindow() {println!("SDL_WarpMouseInWindow")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_PauseAudio() {println!("SDL_PauseAudio")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_CreateThread() {println!("SDL_CreateThread")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetModState() {println!("SDL_GetModState")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetDisplayMode() {println!("SDL_GetDisplayMode")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetRenderDriverInfo() {println!("SDL_GetRenderDriverInfo")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_Init() {println!("SDL_Init")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GameControllerOpen() {println!("SDL_GameControllerOpen")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_InitSubSystem() {println!("SDL_InitSubSystem")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_FreeSurface() {println!("SDL_FreeSurface")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_RenderClear() {println!("SDL_RenderClear")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickGetAxis() {println!("SDL_JoystickGetAxis")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickNumBalls() {println!("SDL_JoystickNumBalls")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_CreateRGBSurfaceFrom() {println!("SDL_CreateRGBSurfaceFrom")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GameControllerGetButton() {println!("SDL_GameControllerGetButton")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetTicks() {println!("SDL_GetTicks")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GameControllerClose() {println!("SDL_GameControllerClose")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickNumButtons() {println!("SDL_JoystickNumButtons")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_RenderCopy() {println!("SDL_RenderCopy")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_UpdateWindowSurface() {println!("SDL_UpdateWindowSurface")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetKeyFromScancode() {println!("SDL_GetKeyFromScancode")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_SetWindowIcon() {println!("SDL_SetWindowIcon")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GameControllerNameForIndex() {println!("SDL_GameControllerNameForIndex")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetNumDisplayModes() {println!("SDL_GetNumDisplayModes")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetWindowSurface() {println!("SDL_GetWindowSurface")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickNameForIndex() {println!("SDL_JoystickNameForIndex")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_LockSurface() {println!("SDL_LockSurface")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetNumRenderDrivers() {println!("SDL_GetNumRenderDrivers")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_UnlockSurface() {println!("SDL_UnlockSurface")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_SetRenderDrawColor() {println!("SDL_SetRenderDrawColor")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetRendererInfo() {println!("SDL_GetRendererInfo")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetWindowPosition() {println!("SDL_GetWindowPosition")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_DetachThread() {println!("SDL_DetachThread")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_OpenAudio() {println!("SDL_OpenAudio")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickGetButton() {println!("SDL_JoystickGetButton")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetMouseState() {println!("SDL_GetMouseState")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GameControllerMapping() {println!("SDL_GameControllerMapping")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetDesktopDisplayMode() {println!("SDL_GetDesktopDisplayMode")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_ClearError() {println!("SDL_ClearError")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetWindowDisplayIndex() {println!("SDL_GetWindowDisplayIndex")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GL_SetAttribute() {println!("SDL_GL_SetAttribute")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_SetHint() {println!("SDL_SetHint")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_SetWindowTitle() {println!("SDL_SetWindowTitle")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetAudioDriver() {println!("SDL_GetAudioDriver")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_SetClipboardText() {println!("SDL_SetClipboardText")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_IsGameController() {println!("SDL_IsGameController")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_HasClipboardText() {println!("SDL_HasClipboardText")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GameControllerGetJoystick() {println!("SDL_GameControllerGetJoystick")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_CreateTexture() {println!("SDL_CreateTexture")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetVideoDriver() {println!("SDL_GetVideoDriver")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_RaiseWindow() {println!("SDL_RaiseWindow")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_ShowCursor() {println!("SDL_ShowCursor")}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetKeyboardState() {println!("SDL_GetKeyboardState")}
