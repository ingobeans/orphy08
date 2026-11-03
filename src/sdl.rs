// all the SDL2 functions that will need to be implemented

#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_DestroyWindow() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_CreateRenderer() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_CreateWindow() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetNumVideoDisplays() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_RenderPresent() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GL_DeleteContext() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetCurrentAudioDriver() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_LockAudio() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetWindowID() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetNumAudioDrivers() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_UnlockAudio() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetDisplayBounds() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GL_CreateContext() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetClipboardText() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetWindowSize() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_JoystickInstanceID() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetVersion() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GameControllerGetAttached() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_DestroyRenderer() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_JoystickClose() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_JoystickOpen() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_JoystickNumAxes() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GameControllerGetAxis() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetDisplayName() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetScancodeName() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_free() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_SetWindowGrab() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetCurrentDisplayMode() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetCurrentVideoDriver() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetNumVideoDrivers() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_PollEvent() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetKeyboardFocus() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_NumJoysticks() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GameControllerAddMapping() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_DestroyTexture() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_Delay() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetError() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_UpdateTexture() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_WarpMouseInWindow() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_PauseAudio() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_CreateThread() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetModState() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetDisplayMode() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetRenderDriverInfo() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_Init() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GameControllerOpen() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_InitSubSystem() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_FreeSurface() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_RenderClear() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_JoystickGetAxis() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_JoystickNumBalls() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_CreateRGBSurfaceFrom() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GameControllerGetButton() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetTicks() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GameControllerClose() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_JoystickNumButtons() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_RenderCopy() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_UpdateWindowSurface() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetKeyFromScancode() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_SetWindowIcon() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GameControllerNameForIndex() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetNumDisplayModes() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetWindowSurface() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_JoystickNameForIndex() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_LockSurface() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetNumRenderDrivers() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_UnlockSurface() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_SetRenderDrawColor() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetRendererInfo() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetWindowPosition() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_DetachThread() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_OpenAudio() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_JoystickGetButton() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetMouseState() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GameControllerMapping() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetDesktopDisplayMode() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_ClearError() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetWindowDisplayIndex() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GL_SetAttribute() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_SetHint() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_SetWindowTitle() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetAudioDriver() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_SetClipboardText() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_IsGameController() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_HasClipboardText() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GameControllerGetJoystick() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_CreateTexture() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetVideoDriver() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_RaiseWindow() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_ShowCursor() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn SDL_GetKeyboardState() {}
