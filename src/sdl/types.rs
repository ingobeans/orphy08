#![allow(non_camel_case_types)]

use enum_iterator::Sequence;
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

pub type SDL_SurfaceFlags = u32;
pub type SDL_SurfaceDataFlags = u32;
pub type SDL_PropertiesID = u32;

#[repr(C)]
pub struct SDL_Surface {
    // Public API definition
    /** The flags of the surface, read-only */
    flags: SDL_SurfaceFlags,
    /** The format of the surface, read-only */
    format: SDL_PixelFormat,
    /** The width of the surface, read-only. */
    w: i32,
    /** The height of the surface, read-only. */
    h: i32,
    /** The distance in bytes between rows of pixels, read-only */
    pitch: i32,
    /** A pointer to the pixels of the surface, the pixels are writeable if non-NULL */
    pixels: *const c_void,

    /** Application reference count, used when freeing surface */
    refcount: i32,

    /** Reserved for internal use */
    reserved: *const c_void,

    // Private API definition
    /** flags for this surface */
    internal_flags: SDL_SurfaceDataFlags,

    /** properties for this surface */
    props: SDL_PropertiesID,

    /** detailed format for this surface */
    fmt: *const c_void, //SDL_PixelFormatDetails *,

    /** Pixel colorspace */
    colorspace: SDL_Colorspace,

    /** palette for indexed surfaces */
    palette: *const c_void, //SDL_Palette *,

    /** Alternate representation of images */
    num_images: i32,
    images: *const c_void, //SDL_Surface **,

    /** information needed for surfaces requiring locks */
    locked: i32,

    /** clipping information */
    clip_rect: SDL_Rect,

    /** info for fast blit mapping to other surfaces */
    map: SDL_BlitMap,

    /** Original pixels when RLE is enabled */
    saved_pixels: *const c_void,
}

unsafe impl Sync for SDL_Surface {}
unsafe impl Send for SDL_Surface {}

#[repr(C)]
pub struct SDL_BlitInfo {
    src_surface: *const SDL_Surface,
    src: *const u8,
    src_h: i32,
    src_w: i32,
    src_pitch: i32,
    src_skip: i32,
    leading_skip: i32,
    dst_surface: *const SDL_Surface,
    dst: *const u8,
    dst_h: i32,
    dst_w: i32,
    dst_pitch: i32,
    dst_skip: i32,
    src_fmt: *const c_void,
    src_pal: *const c_void,
    dst_fmt: *const c_void,
    dst_pal: *const c_void,
    table: *const u8,
    palette_map: *const c_void,
    flags: i32,
    colorkey: u32,
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

#[repr(C)]
pub struct SDL_BlitMap {
    identity: i32,
    blit: *const c_void,
    data: *const c_void,
    info: SDL_BlitInfo,

    dst_palette_version: u32,
    src_palette_version: u32,
}

#[repr(C)]
pub struct SDL_Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[repr(C)]
pub enum SDL_Colorspace {
    SDL_COLORSPACE_UNKNOWN = 0,

    /* sRGB is a gamma corrected colorspace, and the default colorspace for SDL rendering and 8-bit RGB surfaces */
    /** Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709 */
    SDL_COLORSPACE_SRGB = 0x120005a0,
    /* SDL_DEFINE_COLORSPACE(SDL_COLOR_TYPE_RGB,
                                 SDL_COLOR_RANGE_FULL,
                                 SDL_COLOR_PRIMARIES_BT709,
                                 SDL_TRANSFER_CHARACTERISTICS_SRGB,
                                 SDL_MATRIX_COEFFICIENTS_IDENTITY,
                                 SDL_CHROMA_LOCATION_NONE), */

    /* This is a linear colorspace and the default colorspace for floating point surfaces. On Windows this is the scRGB colorspace, and on Apple platforms this is kCGColorSpaceExtendedLinearSRGB for EDR content */
    /** Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709  */
    SDL_COLORSPACE_SRGB_LINEAR = 0x12000500,
    /* SDL_DEFINE_COLORSPACE(SDL_COLOR_TYPE_RGB,
                                 SDL_COLOR_RANGE_FULL,
                                 SDL_COLOR_PRIMARIES_BT709,
                                 SDL_TRANSFER_CHARACTERISTICS_LINEAR,
                                 SDL_MATRIX_COEFFICIENTS_IDENTITY,
                                 SDL_CHROMA_LOCATION_NONE), */

    /* HDR10 is a non-linear HDR colorspace and the default colorspace for 10-bit surfaces */
    /** Equivalent to DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020  */
    SDL_COLORSPACE_HDR10 = 0x12002600,
    /* SDL_DEFINE_COLORSPACE(SDL_COLOR_TYPE_RGB,
    SDL_COLOR_RANGE_FULL,
    SDL_COLOR_PRIMARIES_BT2020,
    SDL_TRANSFER_CHARACTERISTICS_PQ,
    SDL_MATRIX_COEFFICIENTS_IDENTITY,
    SDL_CHROMA_LOCATION_NONE), */
    /** Equivalent to DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601 */
    SDL_COLORSPACE_JPEG = 0x220004c6,
    /* SDL_DEFINE_COLORSPACE(SDL_COLOR_TYPE_YCBCR,
    SDL_COLOR_RANGE_FULL,
    SDL_COLOR_PRIMARIES_BT709,
    SDL_TRANSFER_CHARACTERISTICS_BT601,
    SDL_MATRIX_COEFFICIENTS_BT601,
    SDL_CHROMA_LOCATION_NONE), */
    /** Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601 */
    SDL_COLORSPACE_BT601_LIMITED = 0x211018c6,
    /* SDL_DEFINE_COLORSPACE(SDL_COLOR_TYPE_YCBCR,
    SDL_COLOR_RANGE_LIMITED,
    SDL_COLOR_PRIMARIES_BT601,
    SDL_TRANSFER_CHARACTERISTICS_BT601,
    SDL_MATRIX_COEFFICIENTS_BT601,
    SDL_CHROMA_LOCATION_LEFT), */
    /** Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601 */
    SDL_COLORSPACE_BT601_FULL = 0x221018c6,
    /* SDL_DEFINE_COLORSPACE(SDL_COLOR_TYPE_YCBCR,
    SDL_COLOR_RANGE_FULL,
    SDL_COLOR_PRIMARIES_BT601,
    SDL_TRANSFER_CHARACTERISTICS_BT601,
    SDL_MATRIX_COEFFICIENTS_BT601,
    SDL_CHROMA_LOCATION_LEFT), */
    /** Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709 */
    SDL_COLORSPACE_BT709_LIMITED = 0x21100421,
    /* SDL_DEFINE_COLORSPACE(SDL_COLOR_TYPE_YCBCR,
    SDL_COLOR_RANGE_LIMITED,
    SDL_COLOR_PRIMARIES_BT709,
    SDL_TRANSFER_CHARACTERISTICS_BT709,
    SDL_MATRIX_COEFFICIENTS_BT709,
    SDL_CHROMA_LOCATION_LEFT), */
    /** Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709 */
    SDL_COLORSPACE_BT709_FULL = 0x22100421,
    /* SDL_DEFINE_COLORSPACE(SDL_COLOR_TYPE_YCBCR,
    SDL_COLOR_RANGE_FULL,
    SDL_COLOR_PRIMARIES_BT709,
    SDL_TRANSFER_CHARACTERISTICS_BT709,
    SDL_MATRIX_COEFFICIENTS_BT709,
    SDL_CHROMA_LOCATION_LEFT), */
    /** Equivalent to DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020 */
    SDL_COLORSPACE_BT2020_LIMITED = 0x21102609,
    /* SDL_DEFINE_COLORSPACE(SDL_COLOR_TYPE_YCBCR,
    SDL_COLOR_RANGE_LIMITED,
    SDL_COLOR_PRIMARIES_BT2020,
    SDL_TRANSFER_CHARACTERISTICS_PQ,
    SDL_MATRIX_COEFFICIENTS_BT2020_NCL,
    SDL_CHROMA_LOCATION_LEFT), */
    /** Equivalent to DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020 */
    SDL_COLORSPACE_BT2020_FULL = 0x22102609,
    /* SDL_DEFINE_COLORSPACE(SDL_COLOR_TYPE_YCBCR,
    SDL_COLOR_RANGE_FULL,
    SDL_COLOR_PRIMARIES_BT2020,
    SDL_TRANSFER_CHARACTERISTICS_PQ,
    SDL_MATRIX_COEFFICIENTS_BT2020_NCL,
    SDL_CHROMA_LOCATION_LEFT), */
}

#[repr(C)]
pub enum SDL_PixelFormat {
    SDL_PIXELFORMAT_UNKNOWN = 0,
    SDL_PIXELFORMAT_INDEX1LSB = 0x11100100,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX1, SDL_BITMAPORDER_4321, 0, 1, 0), */
    SDL_PIXELFORMAT_INDEX1MSB = 0x11200100,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX1, SDL_BITMAPORDER_1234, 0, 1, 0), */
    SDL_PIXELFORMAT_INDEX2LSB = 0x1c100200,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX2, SDL_BITMAPORDER_4321, 0, 2, 0), */
    SDL_PIXELFORMAT_INDEX2MSB = 0x1c200200,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX2, SDL_BITMAPORDER_1234, 0, 2, 0), */
    SDL_PIXELFORMAT_INDEX4LSB = 0x12100400,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX4, SDL_BITMAPORDER_4321, 0, 4, 0), */
    SDL_PIXELFORMAT_INDEX4MSB = 0x12200400,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX4, SDL_BITMAPORDER_1234, 0, 4, 0), */
    SDL_PIXELFORMAT_INDEX8 = 0x13000801,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX8, 0, 0, 8, 1), */
    SDL_PIXELFORMAT_RGB332 = 0x14110801,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED8, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_332, 8, 1), */
    SDL_PIXELFORMAT_XRGB4444 = 0x15120c02,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_4444, 12, 2), */
    SDL_PIXELFORMAT_XBGR4444 = 0x15520c02,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_4444, 12, 2), */
    SDL_PIXELFORMAT_XRGB1555 = 0x15130f02,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_1555, 15, 2), */
    SDL_PIXELFORMAT_XBGR1555 = 0x15530f02,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_1555, 15, 2), */
    SDL_PIXELFORMAT_ARGB4444 = 0x15321002,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_4444, 16, 2), */
    SDL_PIXELFORMAT_RGBA4444 = 0x15421002,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_4444, 16, 2), */
    SDL_PIXELFORMAT_ABGR4444 = 0x15721002,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_4444, 16, 2), */
    SDL_PIXELFORMAT_BGRA4444 = 0x15821002,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_4444, 16, 2), */
    SDL_PIXELFORMAT_ARGB1555 = 0x15331002,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_1555, 16, 2), */
    SDL_PIXELFORMAT_RGBA5551 = 0x15441002,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_5551, 16, 2), */
    SDL_PIXELFORMAT_ABGR1555 = 0x15731002,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_1555, 16, 2), */
    SDL_PIXELFORMAT_BGRA5551 = 0x15841002,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_5551, 16, 2), */
    SDL_PIXELFORMAT_RGB565 = 0x15151002,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_565, 16, 2), */
    SDL_PIXELFORMAT_BGR565 = 0x15551002,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_565, 16, 2), */
    SDL_PIXELFORMAT_RGB24 = 0x17101803,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU8, SDL_ARRAYORDER_RGB, 0, 24, 3), */
    SDL_PIXELFORMAT_BGR24 = 0x17401803,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU8, SDL_ARRAYORDER_BGR, 0, 24, 3), */
    SDL_PIXELFORMAT_XRGB8888 = 0x16161804,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_8888, 24, 4), */
    SDL_PIXELFORMAT_RGBX8888 = 0x16261804,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_RGBX, SDL_PACKEDLAYOUT_8888, 24, 4), */
    SDL_PIXELFORMAT_XBGR8888 = 0x16561804,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_8888, 24, 4), */
    SDL_PIXELFORMAT_BGRX8888 = 0x16661804,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_BGRX, SDL_PACKEDLAYOUT_8888, 24, 4), */
    SDL_PIXELFORMAT_ARGB8888 = 0x16362004,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_8888, 32, 4), */
    SDL_PIXELFORMAT_RGBA8888 = 0x16462004,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_8888, 32, 4), */
    SDL_PIXELFORMAT_ABGR8888 = 0x16762004,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_8888, 32, 4), */
    SDL_PIXELFORMAT_BGRA8888 = 0x16862004,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_8888, 32, 4), */
    SDL_PIXELFORMAT_XRGB2101010 = 0x16172004,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_2101010, 32, 4), */
    SDL_PIXELFORMAT_XBGR2101010 = 0x16572004,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_2101010, 32, 4), */
    SDL_PIXELFORMAT_ARGB2101010 = 0x16372004,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_2101010, 32, 4), */
    SDL_PIXELFORMAT_ABGR2101010 = 0x16772004,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_2101010, 32, 4), */
    SDL_PIXELFORMAT_RGB48 = 0x18103006,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_RGB, 0, 48, 6), */
    SDL_PIXELFORMAT_BGR48 = 0x18403006,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_BGR, 0, 48, 6), */
    SDL_PIXELFORMAT_RGBA64 = 0x18204008,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_RGBA, 0, 64, 8), */
    SDL_PIXELFORMAT_ARGB64 = 0x18304008,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_ARGB, 0, 64, 8), */
    SDL_PIXELFORMAT_BGRA64 = 0x18504008,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_BGRA, 0, 64, 8), */
    SDL_PIXELFORMAT_ABGR64 = 0x18604008,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_ABGR, 0, 64, 8), */
    SDL_PIXELFORMAT_RGB48_FLOAT = 0x1a103006,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_RGB, 0, 48, 6), */
    SDL_PIXELFORMAT_BGR48_FLOAT = 0x1a403006,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_BGR, 0, 48, 6), */
    SDL_PIXELFORMAT_RGBA64_FLOAT = 0x1a204008,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_RGBA, 0, 64, 8), */
    SDL_PIXELFORMAT_ARGB64_FLOAT = 0x1a304008,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_ARGB, 0, 64, 8), */
    SDL_PIXELFORMAT_BGRA64_FLOAT = 0x1a504008,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_BGRA, 0, 64, 8), */
    SDL_PIXELFORMAT_ABGR64_FLOAT = 0x1a604008,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_ABGR, 0, 64, 8), */
    SDL_PIXELFORMAT_RGB96_FLOAT = 0x1b10600c,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_RGB, 0, 96, 12), */
    SDL_PIXELFORMAT_BGR96_FLOAT = 0x1b40600c,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_BGR, 0, 96, 12), */
    SDL_PIXELFORMAT_RGBA128_FLOAT = 0x1b208010,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_RGBA, 0, 128, 16), */
    SDL_PIXELFORMAT_ARGB128_FLOAT = 0x1b308010,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_ARGB, 0, 128, 16), */
    SDL_PIXELFORMAT_BGRA128_FLOAT = 0x1b508010,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_BGRA, 0, 128, 16), */
    SDL_PIXELFORMAT_ABGR128_FLOAT = 0x1b608010,
    /* SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_ABGR, 0, 128, 16), */
    /** YUV 4:2:0 8-bit planar mode: Y + V + U  (3 planes) */
    SDL_PIXELFORMAT_YV12 = 0x32315659,
    /* SDL_DEFINE_PIXELFOURCC('Y', 'V', '1', '2'), */
    /** YUV 4:2:0 8-bit planar mode: Y + U + V  (3 planes) */
    SDL_PIXELFORMAT_IYUV = 0x56555949,
    /* SDL_DEFINE_PIXELFOURCC('I', 'Y', 'U', 'V'), */
    /** YUV 4:2:0 8-bit packed mode: Y0+U0+Y1+V0 (1 plane) */
    SDL_PIXELFORMAT_YUY2 = 0x32595559,
    /* SDL_DEFINE_PIXELFOURCC('Y', 'U', 'Y', '2'), */
    /** YUV 4:2:0 8-bit packed mode: U0+Y0+V0+Y1 (1 plane) */
    SDL_PIXELFORMAT_UYVY = 0x59565955,
    /* SDL_DEFINE_PIXELFOURCC('U', 'Y', 'V', 'Y'), */
    /** YUV 4:2:0 8-bit packed mode: Y0+V0+Y1+U0 (1 plane) */
    SDL_PIXELFORMAT_YVYU = 0x55595659,
    /* SDL_DEFINE_PIXELFOURCC('Y', 'V', 'Y', 'U'), */
    /** YUV 4:2:0 8-bit planar mode: Y + U/V interleaved  (2 planes) */
    SDL_PIXELFORMAT_NV12 = 0x3231564e,
    /* SDL_DEFINE_PIXELFOURCC('N', 'V', '1', '2'), */
    /** YUV 4:2:0 8-bit planar mode: Y + V/U interleaved  (2 planes) */
    SDL_PIXELFORMAT_NV21 = 0x3132564e,
    /* SDL_DEFINE_PIXELFOURCC('N', 'V', '2', '1'), */
    /** YUV 4:4:4 8-bit planar mode: Y + U + V  (3 planes) */
    SDL_PIXELFORMAT_I444 = 0x34343449,
    /* SDL_DEFINE_PIXELFOURCC('I', '4', '4', '4'), */
    /** YUV 4:2:0 16-bit planar mode: Y + U/V interleaved  (2 planes) */
    SDL_PIXELFORMAT_P010 = 0x30313050,
    /* SDL_DEFINE_PIXELFOURCC('P', '0', '1', '0'), */
    /** YUV 4:2:0 16-bit planar mode: Y + U + V  (3 planes) */
    SDL_PIXELFORMAT_I0FL = 0x4c463049,
    /* SDL_DEFINE_PIXELFOURCC('I', '0', 'F', 'L'), */
    /** YUV 4:4:4 16-bit planar mode: Y + U + V  (3 planes) */
    SDL_PIXELFORMAT_I4FL = 0x4c463449,
    /* SDL_DEFINE_PIXELFOURCC('I', '4', 'F', 'L'), */
    /** Android video texture format */
    SDL_PIXELFORMAT_EXTERNAL_OES = 0x2053454f,
    /* SDL_DEFINE_PIXELFOURCC('O', 'E', 'S', ' ') */
    /** Motion JPEG */
    SDL_PIXELFORMAT_MJPG = 0x47504a4d,
    /* SDL_DEFINE_PIXELFOURCC('M', 'J', 'P', 'G') */

    /* Aliases for RGBA byte arrays of color data, for the current platform */
    // SDL_PIXELFORMAT_RGBA32 = SDL_PIXELFORMAT_ABGR8888,
    // SDL_PIXELFORMAT_ARGB32 = SDL_PIXELFORMAT_BGRA8888,
    // SDL_PIXELFORMAT_BGRA32 = SDL_PIXELFORMAT_ARGB8888,
    // SDL_PIXELFORMAT_ABGR32 = SDL_PIXELFORMAT_RGBA8888,
    // SDL_PIXELFORMAT_RGBX32 = SDL_PIXELFORMAT_XBGR8888,
    // SDL_PIXELFORMAT_XRGB32 = SDL_PIXELFORMAT_BGRX8888,
    // SDL_PIXELFORMAT_BGRX32 = SDL_PIXELFORMAT_XRGB8888,
    // SDL_PIXELFORMAT_XBGR32 = SDL_PIXELFORMAT_RGBX8888
}

#[derive(Sequence)]
#[repr(C)]
pub enum SDL_Scancode {
    SDL_SCANCODE_UNKNOWN = 0,

    /**
     *  \name Usage page 0x07
     *
     *  These values are from usage page 0x07 (USB keyboard page).
     */
    /* @{ */
    SDL_SCANCODE_A = 4,
    SDL_SCANCODE_B = 5,
    SDL_SCANCODE_C = 6,
    SDL_SCANCODE_D = 7,
    SDL_SCANCODE_E = 8,
    SDL_SCANCODE_F = 9,
    SDL_SCANCODE_G = 10,
    SDL_SCANCODE_H = 11,
    SDL_SCANCODE_I = 12,
    SDL_SCANCODE_J = 13,
    SDL_SCANCODE_K = 14,
    SDL_SCANCODE_L = 15,
    SDL_SCANCODE_M = 16,
    SDL_SCANCODE_N = 17,
    SDL_SCANCODE_O = 18,
    SDL_SCANCODE_P = 19,
    SDL_SCANCODE_Q = 20,
    SDL_SCANCODE_R = 21,
    SDL_SCANCODE_S = 22,
    SDL_SCANCODE_T = 23,
    SDL_SCANCODE_U = 24,
    SDL_SCANCODE_V = 25,
    SDL_SCANCODE_W = 26,
    SDL_SCANCODE_X = 27,
    SDL_SCANCODE_Y = 28,
    SDL_SCANCODE_Z = 29,

    SDL_SCANCODE_1 = 30,
    SDL_SCANCODE_2 = 31,
    SDL_SCANCODE_3 = 32,
    SDL_SCANCODE_4 = 33,
    SDL_SCANCODE_5 = 34,
    SDL_SCANCODE_6 = 35,
    SDL_SCANCODE_7 = 36,
    SDL_SCANCODE_8 = 37,
    SDL_SCANCODE_9 = 38,
    SDL_SCANCODE_0 = 39,

    SDL_SCANCODE_RETURN = 40,
    SDL_SCANCODE_ESCAPE = 41,
    SDL_SCANCODE_BACKSPACE = 42,
    SDL_SCANCODE_TAB = 43,
    SDL_SCANCODE_SPACE = 44,

    SDL_SCANCODE_MINUS = 45,
    SDL_SCANCODE_EQUALS = 46,
    SDL_SCANCODE_LEFTBRACKET = 47,
    SDL_SCANCODE_RIGHTBRACKET = 48,
    /** Located at the lower left of the returnSDL_SCANCODE_BACKSLASH = 49,
     *   key on ISO keyboards and at the right end
     *   of the QWERTY row on ANSI keyboards.
     *   Produces REVERSE SOLIDUS (backslash) and
     *   VERTICAL LINE in a US layout, REVERSE
     *   SOLIDUS and VERTICAL LINE in a UK Mac
     *   layout, NUMBER SIGN and TILDE in a UK
     *   Windows layout, DOLLAR SIGN and POUND SIGN
     *   in a Swiss German layout, NUMBER SIGN and
     *   APOSTROPHE in a German layout, GRAVE
     *   ACCENT and POUND SIGN in a French Mac
     *   layout, and ASTERISK and MICRO SIGN in a
     *   French Windows layout.
     */
    /** ISO USB keyboards actually use this codeSDL_SCANCODE_NONUSHASH = 50,
     *   instead of 49 for the same key, but all
     *   OSes I've seen treat the two codes
     *   identically. So, as an implementor, unless
     *   your keyboard generates both of those
     *   codes and your OS treats them differently,
     *   you should generate SDL_SCANCODE_BACKSLASH
     *   instead of this code. As a user, you
     *   should not rely on this code because SDL
     *   will never generate it with most (all?)
     *   keyboards.
     */
    SDL_SCANCODE_SEMICOLON = 51,
    SDL_SCANCODE_APOSTROPHE = 52,
    /** Located in the top left corner (on both ANSISDL_SCANCODE_GRAVE = 53,
     *   and ISO keyboards). Produces GRAVE ACCENT and
     *   TILDE in a US Windows layout and in US and UK
     *   Mac layouts on ANSI keyboards, GRAVE ACCENT
     *   and NOT SIGN in a UK Windows layout, SECTION
     *   SIGN and PLUS-MINUS SIGN in US and UK Mac
     *   layouts on ISO keyboards, SECTION SIGN and
     *   DEGREE SIGN in a Swiss German layout (Mac:
     *   only on ISO keyboards), CIRCUMFLEX ACCENT and
     *   DEGREE SIGN in a German layout (Mac: only on
     *   ISO keyboards), SUPERSCRIPT TWO and TILDE in a
     *   French Windows layout, COMMERCIAL AT and
     *   NUMBER SIGN in a French Mac layout on ISO
     *   keyboards, and LESS-THAN SIGN and GREATER-THAN
     *   SIGN in a Swiss German, German, or French Mac
     *   layout on ANSI keyboards.
     */
    SDL_SCANCODE_COMMA = 54,
    SDL_SCANCODE_PERIOD = 55,
    SDL_SCANCODE_SLASH = 56,

    SDL_SCANCODE_CAPSLOCK = 57,

    SDL_SCANCODE_F1 = 58,
    SDL_SCANCODE_F2 = 59,
    SDL_SCANCODE_F3 = 60,
    SDL_SCANCODE_F4 = 61,
    SDL_SCANCODE_F5 = 62,
    SDL_SCANCODE_F6 = 63,
    SDL_SCANCODE_F7 = 64,
    SDL_SCANCODE_F8 = 65,
    SDL_SCANCODE_F9 = 66,
    SDL_SCANCODE_F10 = 67,
    SDL_SCANCODE_F11 = 68,
    SDL_SCANCODE_F12 = 69,

    SDL_SCANCODE_PRINTSCREEN = 70,
    SDL_SCANCODE_SCROLLLOCK = 71,
    SDL_SCANCODE_PAUSE = 72,
    /** insert on PC, help on some Mac keyboards (butSDL_SCANCODE_INSERT = 73,
    does send code 73, not 117) */
    SDL_SCANCODE_HOME = 74,
    SDL_SCANCODE_PAGEUP = 75,
    SDL_SCANCODE_DELETE = 76,
    SDL_SCANCODE_END = 77,
    SDL_SCANCODE_PAGEDOWN = 78,
    SDL_SCANCODE_RIGHT = 79,
    SDL_SCANCODE_LEFT = 80,
    SDL_SCANCODE_DOWN = 81,
    SDL_SCANCODE_UP = 82,

    /** num lock on PC, clear on Mac keyboardsSDL_SCANCODE_NUMLOCKCLEAR = 83,
     */
    SDL_SCANCODE_KP_DIVIDE = 84,
    SDL_SCANCODE_KP_MULTIPLY = 85,
    SDL_SCANCODE_KP_MINUS = 86,
    SDL_SCANCODE_KP_PLUS = 87,
    SDL_SCANCODE_KP_ENTER = 88,
    SDL_SCANCODE_KP_1 = 89,
    SDL_SCANCODE_KP_2 = 90,
    SDL_SCANCODE_KP_3 = 91,
    SDL_SCANCODE_KP_4 = 92,
    SDL_SCANCODE_KP_5 = 93,
    SDL_SCANCODE_KP_6 = 94,
    SDL_SCANCODE_KP_7 = 95,
    SDL_SCANCODE_KP_8 = 96,
    SDL_SCANCODE_KP_9 = 97,
    SDL_SCANCODE_KP_0 = 98,
    SDL_SCANCODE_KP_PERIOD = 99,

    /** This is the additional key that ISOSDL_SCANCODE_NONUSBACKSLASH = 100,
     *   keyboards have over ANSI ones,
     *   located between left shift and Y.
     *   Produces GRAVE ACCENT and TILDE in a
     *   US or UK Mac layout, REVERSE SOLIDUS
     *   (backslash) and VERTICAL LINE in a
     *   US or UK Windows layout, and
     *   LESS-THAN SIGN and GREATER-THAN SIGN
     *   in a Swiss German, German, or French
     *   layout. */
    /** windows contextual men, compose */
    SDL_SCANCODE_APPLICATION = 101,
    /** The USB document says this is a status flag,SDL_SCANCODE_POWER = 102,
     *   not a physical key - but some Mac keyboards
     *   do have a power key. */
    SDL_SCANCODE_KP_EQUALS = 103,
    SDL_SCANCODE_F13 = 104,
    SDL_SCANCODE_F14 = 105,
    SDL_SCANCODE_F15 = 106,
    SDL_SCANCODE_F16 = 107,
    SDL_SCANCODE_F17 = 108,
    SDL_SCANCODE_F18 = 109,
    SDL_SCANCODE_F19 = 110,
    SDL_SCANCODE_F20 = 111,
    SDL_SCANCODE_F21 = 112,
    SDL_SCANCODE_F22 = 113,
    SDL_SCANCODE_F23 = 114,
    SDL_SCANCODE_F24 = 115,
    SDL_SCANCODE_EXECUTE = 116,
    /** AL Integrated Help Center */
    SDL_SCANCODE_HELP = 117,
    /** Menu (show menu) */
    SDL_SCANCODE_MENU = 118,
    SDL_SCANCODE_SELECT = 119,
    /** AC Stop */
    SDL_SCANCODE_STOP = 120,
    /** AC Redo/Repeat */
    SDL_SCANCODE_AGAIN = 121,
    /** AC Undo */
    SDL_SCANCODE_UNDO = 122,
    /** AC Cut */
    SDL_SCANCODE_CUT = 123,
    /** AC Copy */
    SDL_SCANCODE_COPY = 124,
    /** AC Paste */
    SDL_SCANCODE_PASTE = 125,
    /** AC Find */
    SDL_SCANCODE_FIND = 126,
    SDL_SCANCODE_MUTE = 127,
    SDL_SCANCODE_VOLUMEUP = 128,
    SDL_SCANCODE_VOLUMEDOWN = 129,
    /* not sure whether there's a reason to enable these */
    /*     SDL_SCANCODE_LOCKINGCAPSLOCK = 130,  */
    /*     SDL_SCANCODE_LOCKINGNUMLOCK = 131, */
    /*     SDL_SCANCODE_LOCKINGSCROLLLOCK = 132, */
    SDL_SCANCODE_KP_COMMA = 133,
    SDL_SCANCODE_KP_EQUALSAS400 = 134,

    /** used on Asian keyboards, seeSDL_SCANCODE_INTERNATIONAL1 = 135,
    footnotes in USB doc */
    SDL_SCANCODE_INTERNATIONAL2 = 136,
    /** Yen */
    SDL_SCANCODE_INTERNATIONAL3 = 137,
    SDL_SCANCODE_INTERNATIONAL4 = 138,
    SDL_SCANCODE_INTERNATIONAL5 = 139,
    SDL_SCANCODE_INTERNATIONAL6 = 140,
    SDL_SCANCODE_INTERNATIONAL7 = 141,
    SDL_SCANCODE_INTERNATIONAL8 = 142,
    SDL_SCANCODE_INTERNATIONAL9 = 143,
    /** Hangul/English toggle */
    SDL_SCANCODE_LANG1 = 144,
    /** Hanja conversion */
    SDL_SCANCODE_LANG2 = 145,
    /** Katakana */
    SDL_SCANCODE_LANG3 = 146,
    /** Hiragana */
    SDL_SCANCODE_LANG4 = 147,
    /** Zenkaku/Hankaku */
    SDL_SCANCODE_LANG5 = 148,
    /** reserved */
    SDL_SCANCODE_LANG6 = 149,
    /** reserved */
    SDL_SCANCODE_LANG7 = 150,
    /** reserved */
    SDL_SCANCODE_LANG8 = 151,
    /** reserved */
    SDL_SCANCODE_LANG9 = 152,

    /** Erase-Eaze */
    SDL_SCANCODE_ALTERASE = 153,
    SDL_SCANCODE_SYSREQ = 154,
    /** AC Cancel */
    SDL_SCANCODE_CANCEL = 155,
    SDL_SCANCODE_CLEAR = 156,
    SDL_SCANCODE_PRIOR = 157,
    SDL_SCANCODE_RETURN2 = 158,
    SDL_SCANCODE_SEPARATOR = 159,
    SDL_SCANCODE_OUT = 160,
    SDL_SCANCODE_OPER = 161,
    SDL_SCANCODE_CLEARAGAIN = 162,
    SDL_SCANCODE_CRSEL = 163,
    SDL_SCANCODE_EXSEL = 164,

    SDL_SCANCODE_KP_00 = 176,
    SDL_SCANCODE_KP_000 = 177,
    SDL_SCANCODE_THOUSANDSSEPARATOR = 178,
    SDL_SCANCODE_DECIMALSEPARATOR = 179,
    SDL_SCANCODE_CURRENCYUNIT = 180,
    SDL_SCANCODE_CURRENCYSUBUNIT = 181,
    SDL_SCANCODE_KP_LEFTPAREN = 182,
    SDL_SCANCODE_KP_RIGHTPAREN = 183,
    SDL_SCANCODE_KP_LEFTBRACE = 184,
    SDL_SCANCODE_KP_RIGHTBRACE = 185,
    SDL_SCANCODE_KP_TAB = 186,
    SDL_SCANCODE_KP_BACKSPACE = 187,
    SDL_SCANCODE_KP_A = 188,
    SDL_SCANCODE_KP_B = 189,
    SDL_SCANCODE_KP_C = 190,
    SDL_SCANCODE_KP_D = 191,
    SDL_SCANCODE_KP_E = 192,
    SDL_SCANCODE_KP_F = 193,
    SDL_SCANCODE_KP_XOR = 194,
    SDL_SCANCODE_KP_POWER = 195,
    SDL_SCANCODE_KP_PERCENT = 196,
    SDL_SCANCODE_KP_LESS = 197,
    SDL_SCANCODE_KP_GREATER = 198,
    SDL_SCANCODE_KP_AMPERSAND = 199,
    SDL_SCANCODE_KP_DBLAMPERSAND = 200,
    SDL_SCANCODE_KP_VERTICALBAR = 201,
    SDL_SCANCODE_KP_DBLVERTICALBAR = 202,
    SDL_SCANCODE_KP_COLON = 203,
    SDL_SCANCODE_KP_HASH = 204,
    SDL_SCANCODE_KP_SPACE = 205,
    SDL_SCANCODE_KP_AT = 206,
    SDL_SCANCODE_KP_EXCLAM = 207,
    SDL_SCANCODE_KP_MEMSTORE = 208,
    SDL_SCANCODE_KP_MEMRECALL = 209,
    SDL_SCANCODE_KP_MEMCLEAR = 210,
    SDL_SCANCODE_KP_MEMADD = 211,
    SDL_SCANCODE_KP_MEMSUBTRACT = 212,
    SDL_SCANCODE_KP_MEMMULTIPLY = 213,
    SDL_SCANCODE_KP_MEMDIVIDE = 214,
    SDL_SCANCODE_KP_PLUSMINUS = 215,
    SDL_SCANCODE_KP_CLEAR = 216,
    SDL_SCANCODE_KP_CLEARENTRY = 217,
    SDL_SCANCODE_KP_BINARY = 218,
    SDL_SCANCODE_KP_OCTAL = 219,
    SDL_SCANCODE_KP_DECIMAL = 220,
    SDL_SCANCODE_KP_HEXADECIMAL = 221,

    SDL_SCANCODE_LCTRL = 224,
    SDL_SCANCODE_LSHIFT = 225,
    /** alt, option */
    SDL_SCANCODE_LALT = 226,
    /** windows, command (apple), meta */
    SDL_SCANCODE_LGUI = 227,
    SDL_SCANCODE_RCTRL = 228,
    SDL_SCANCODE_RSHIFT = 229,
    /** alt gr, option */
    SDL_SCANCODE_RALT = 230,
    /** windows, command (apple), meta */
    SDL_SCANCODE_RGUI = 231,

    /** I'm not sure if this is really not coveredSDL_SCANCODE_MODE = 257,    
     *   by any of the above, but since there's a
     *   special KMOD_MODE for it I'm adding it here
     */

    /* @} *//* Usage page 0x07 */

    /**
     *  \name Usage page 0x0C
     *
     *  These values are mapped from usage page 0x0C (USB consumer page).
     *  See https://usb.org/sites/default/files/hut1_2.pdf
     *
     *  There are way more keys in the spec than we can represent in the
     *  current scancode range, so pick the ones that commonly come up in
     *  real world usage.
     */
    /* @{ */
    SDL_SCANCODE_AUDIONEXT = 258,
    SDL_SCANCODE_AUDIOPREV = 259,
    SDL_SCANCODE_AUDIOSTOP = 260,
    SDL_SCANCODE_AUDIOPLAY = 261,
    SDL_SCANCODE_AUDIOMUTE = 262,
    SDL_SCANCODE_MEDIASELECT = 263,
    /** AL Internet Browser */
    SDL_SCANCODE_WWW = 264,
    SDL_SCANCODE_MAIL = 265,
    /** AL Calculator */
    SDL_SCANCODE_CALCULATOR = 266,
    SDL_SCANCODE_COMPUTER = 267,
    /** AC Search */
    SDL_SCANCODE_AC_SEARCH = 268,
    /** AC Home */
    SDL_SCANCODE_AC_HOME = 269,
    /** AC Back */
    SDL_SCANCODE_AC_BACK = 270,
    /** AC Forward */
    SDL_SCANCODE_AC_FORWARD = 271,
    /** AC Stop */
    SDL_SCANCODE_AC_STOP = 272,
    /** AC Refresh */
    SDL_SCANCODE_AC_REFRESH = 273,
    /** AC Bookmarks */
    SDL_SCANCODE_AC_BOOKMARKS = 274,

    /* @} *//* Usage page 0x0C */
    /**
     *  \name Walther keys
     *
     *  These are values that Christian Walther added (for mac keyboard?).
     */
    /* @{ */
    SDL_SCANCODE_BRIGHTNESSDOWN = 275,
    SDL_SCANCODE_BRIGHTNESSUP = 276,
    /** display mirroring/dual displaySDL_SCANCODE_DISPLAYSWITCH = 277,
    switch, video mode switch */
    SDL_SCANCODE_KBDILLUMTOGGLE = 278,
    SDL_SCANCODE_KBDILLUMDOWN = 279,
    SDL_SCANCODE_KBDILLUMUP = 280,
    SDL_SCANCODE_EJECT = 281,
    /** SC System Sleep */
    SDL_SCANCODE_SLEEP = 282,

    SDL_SCANCODE_APP1 = 283,
    SDL_SCANCODE_APP2 = 284,

    /* @} *//* Walther keys */
    /**
     *  \name Usage page 0x0C (additional media keys)
     *
     *  These values are mapped from usage page 0x0C (USB consumer page).
     */
    /* @{ */
    SDL_SCANCODE_AUDIOREWIND = 285,
    SDL_SCANCODE_AUDIOFASTFORWARD = 286,

    /* @} *//* Usage page 0x0C (additional media keys) */
    /**
     *  \name Mobile keys
     *
     *  These are values that are often used on mobile phones.
     */
    /* @{ */

    /** Usually situated below the display on phones andSDL_SCANCODE_SOFTLEFT = 287,
                                                                                                                                                          used as a multi-function feature key for selecting
                                                                                                                                                          a software defined function shown on the bottom left
                                                                                                                                                          of the display. */
    /** Usually situated below the display on phones andSDL_SCANCODE_SOFTRIGHT = 288,
                                                                                                                                                           used as a multi-function feature key for selecting
                                                                                                                                                           a software defined function shown on the bottom right
                                                                                                                                                           of the display. */
    /** Used for accepting phone calls. */
    SDL_SCANCODE_CALL = 289,
    /** Used for rejecting phone calls. */
    SDL_SCANCODE_ENDCALL = 290,

    /* @} *//* Mobile keys */

    /* Add any other keys here. */
    /** not a key, just marks the number of scancodes
    for array bounds */
    SDL_NUM_SCANCODES = 512,
}
