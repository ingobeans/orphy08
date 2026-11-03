#![allow(non_camel_case_types)]
use std::ffi::c_void;

/**
 *  The structure that defines a display mode
 *
 *  * SDL_GetNumDisplayModes()
 *  * SDL_GetDisplayMode()
 *  * SDL_GetDesktopDisplayMode()
 *  * SDL_GetCurrentDisplayMode()
 *  * SDL_GetClosestDisplayMode()
 *  * SDL_SetWindowDisplayMode()
 *  * SDL_GetWindowDisplayMode()
 */
#[repr(C)]
pub struct SDL_DisplayMode {
    /** pixel format */
    format: u32,
    /** width, in screen coordinates */
    w: i32,
    /** height, in screen coordinates */
    h: i32,
    /** refresh rate (or zero for unspecified) */
    refresh_rate: i32,
    /** driver-specific data, initialize to 0 */
    driverdata: *mut c_void,
}

/**
 *  The type used to identify a window
 *
 *  * SDL_CreateWindow()
 *  * SDL_CreateWindowFrom()
 *  * SDL_DestroyWindow()
 *  * SDL_GetWindowData()
 *  * SDL_GetWindowFlags()
 *  * SDL_GetWindowGrab()
 *  * SDL_GetWindowPosition()
 *  * SDL_GetWindowSize()
 *  * SDL_GetWindowTitle()
 *  * SDL_HideWindow()
 *  * SDL_MaximizeWindow()
 *  * SDL_MinimizeWindow()
 *  * SDL_RaiseWindow()
 *  * SDL_RestoreWindow()
 *  * SDL_SetWindowData()
 *  * SDL_SetWindowFullscreen()
 *  * SDL_SetWindowGrab()
 *  * SDL_SetWindowIcon()
 *  * SDL_SetWindowPosition()
 *  * SDL_SetWindowSize()
 *  * SDL_SetWindowBordered()
 *  * SDL_SetWindowResizable()
 *  * SDL_SetWindowTitle()
 *  * SDL_ShowWindow()
 */
pub struct SDL_Window {}
