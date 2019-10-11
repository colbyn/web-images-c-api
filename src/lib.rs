#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use libc::size_t;
use image::{
    DynamicImage,
    GenericImage,
    GenericImageView,
    Pixel
};




///////////////////////////////////////////////////////////////////////////////
// IMAGE - BASICS - TYPES
///////////////////////////////////////////////////////////////////////////////

/// A decoded dynamic image.
///
/// This data type will either be valid, or invalid (with an error message).
/// So failed operations may return a non-null value containing an error message.
/// See the functions `wi_img_is_ok`, `wi_img_is_err` and `wi_img_get_err_msg`
/// for further details.
/// 
/// Rather than checking for only NULL pointers, use `wi_img_is_err` instead since
/// it’ll check for both null and error states.
/// 
/// Operations on a failed image may either return NULL or propagate with the
/// original error.
pub struct WiImage(Result<DynamicImage, String>);


/// A decoded grayscale image.
/// 
/// Primarily used in more advanced image processing pipelines.
/// Laymen users are probably looking for the `WiImage` type.
///
/// This data type will either be valid, or invalid (with an error message).
/// So failed operations may return a non-null value containing an error message.
/// See the functions `wi_grayimg_is_ok`, `wi_grayimg_is_err` and `wi_grayimg_get_err_msg`
/// for further details.
///
/// Rather than checking for only NULL pointers, use `wi_grayimg_is_err` instead since
/// it’ll check for both null and error states.
/// 
/// Operations on a failed image may either return NULL or propagate with the
/// original error.
pub struct WiGrayImage(Result<image::GrayImage, String>);


/// A decoded grayscale image.
/// 
/// Primarily used in more advanced image processing pipelines.
/// Laymen users are probably looking for the `WiImage` type.
/// 
/// In contrast to `WiGrayImage`, each pixel is a 32-bit unsigned integer,
/// primarily used for representing labeled images/regions.
///
/// This data type will either be valid, or invalid (with an error message).
/// So failed operations may return a non-null value containing an error message.
/// See the functions `wi_grayimg_u32_is_ok`, `wi_grayimg_u32_is_err` and
/// `wi_grayimg_u32_get_err_msg` for further details.
/// 
/// Rather than checking for only NULL pointers, use `wi_grayimg_is_err` instead since
/// it’ll check for both null and error states.
/// 
/// Operations on a failed image may either return NULL or propagate with the
/// original error.
pub struct WiGrayImageU32(Result<imageproc::definitions::Image<image::Luma<u32>>, String>);



///////////////////////////////////////////////////////////////////////////////
// IMAGE - BASICS - UTILS
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_img_is_ok(ctx: *const WiImage) -> c_int {
    if ctx.is_null() {
        return 0;
    }
    match (&*ctx).0 {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern fn wi_img_is_err(ctx: *const WiImage) -> c_int {
    if ctx.is_null() {
        return 1;
    }
    match (&*ctx).0 {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[no_mangle]
pub unsafe extern fn wi_img_get_err_msg(ctx: *const WiImage) -> *const c_char {
    if ctx.is_null() {
        return std::ptr::null();
    }
    match (&*ctx).0 {
        Ok(_) => std::ptr::null(),
        Err(ref x) => {
            let msg = CString::new(x.clone()).expect("CString::new failed");
            msg.as_ptr()
        }
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_is_ok(ctx: *const WiGrayImage) -> c_int {
    if ctx.is_null() {
        return 0;
    }
    match (&*ctx).0 {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_is_err(ctx: *const WiGrayImage) -> c_int {
    if ctx.is_null() {
        return 1;
    }
    match (&*ctx).0 {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_get_err_msg(ctx: *const WiGrayImage) -> *const c_char {
    if ctx.is_null() {
        return std::ptr::null();
    }
    match (&*ctx).0 {
        Ok(_) => std::ptr::null(),
        Err(ref x) => {
            let msg = CString::new(x.clone()).expect("CString::new failed");
            msg.as_ptr()
        }
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_u32_is_ok(ctx: *const WiGrayImageU32) -> c_int {
    if ctx.is_null() {
        return 0;
    }
    match (&*ctx).0 {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_u32_is_err(ctx: *const WiGrayImageU32) -> c_int {
    if ctx.is_null() {
        return 1;
    }
    match (&*ctx).0 {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_u32_get_err_msg(ctx: *const WiGrayImageU32) -> *const c_char {
    if ctx.is_null() {
        return std::ptr::null();
    }
    match (&*ctx).0 {
        Ok(_) => std::ptr::null(),
        Err(ref x) => {
            let msg = CString::new(x.clone()).expect("CString::new failed");
            msg.as_ptr()
        }
    }
}



///////////////////////////////////////////////////////////////////////////////
// BASICS - MEMORY
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_img_clone(ctx: *const WiImage) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.clone()),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_free(ctx: *mut WiImage) {
    if !ctx.is_null() {
        let value = Box::from_raw(ctx);
        std::mem::drop(value);
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_clone(ctx: *const WiGrayImage) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.clone()),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_free(ctx: *mut WiGrayImage) {
    if !ctx.is_null() {
        let value = Box::from_raw(ctx);
        std::mem::drop(value);
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_u32_clone(ctx: *const WiGrayImageU32) -> *mut WiGrayImageU32 {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.clone()),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImageU32(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_u32_free(ctx: *mut WiGrayImageU32) {
    if !ctx.is_null() {
        let value = Box::from_raw(ctx);
        std::mem::drop(value);
    }
}


///////////////////////////////////////////////////////////////////////////////
// BASICS - CONVERSION
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_img_to_luma(ctx: *const WiImage) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.to_luma()),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}


#[no_mangle]
pub unsafe extern fn wi_grayimg_to_img(ctx: *const WiGrayImage) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(::image::DynamicImage::ImageLuma8(x.clone())),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_u32_to_img_with_pretty_labels(ctx: *const WiGrayImageU32) -> *mut WiImage {
    fn random_color_map(keys: HashSet<u32>) -> HashMap<u32, image::Rgb<u8>> {
        use colourado::{Color, ColorPalette, PaletteType};
        let palette = ColorPalette::new(keys.len() as u32, PaletteType::Random, false);
        let mut output: HashMap<u32, image::Rgb<u8>> = HashMap::new();
        for (key, ix) in keys.iter().zip(0 .. keys.len()) {
            let key = key.clone();
            if key == 0 {
                output.insert(key, image::Rgb([0, 0, 0]));
            } else {
                fn convert(x: f32) -> u8 {
                    (x * 255.0) as u8
                }
                let red = convert(palette.colors[ix].red);
                let green = convert(palette.colors[ix].green);
                let blue = convert(palette.colors[ix].blue);

                output.insert(key, image::Rgb([red, green, blue]));
            }
        }
        output
    }
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let go = |width, height, components: &imageproc::definitions::Image<image::Luma<u32>>| {
        let pixels = components
            .pixels()
            .map(|p| p[0]).map(|x| x)
            .collect();
        let debug_colors = random_color_map(pixels);
        let new_image = image::ImageBuffer::from_fn(width, height, |x, y| {
            let px_key = components.get_pixel(x, y).channels()[0];
            let color = debug_colors.get(&px_key).expect("missing color entry");
            color.clone()
        });
        image::DynamicImage::ImageRgb8(new_image)
    };
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(go(x.width(), x.height(), x)),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}



///////////////////////////////////////////////////////////////////////////////
// IMAGE - METHODS
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_img_open(path: *const c_char) -> *mut WiImage {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let result = CStr::from_ptr(path)
        .to_str()
        .map_err(|x| format!("{:?}", x))
        .and_then(|path| {
            ::image::open(path).map_err(|x| format!("{:?}", x))
        });
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_new_luma8_img(width: u32, height: u32) -> *mut WiImage {
    let result = Ok(DynamicImage::new_luma8(width, height));
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_new_rgb8_img(width: u32, height: u32) -> *mut WiImage {
    let result = Ok(DynamicImage::new_rgb8(width, height));
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_new_rgba8_img(width: u32, height: u32) -> *mut WiImage {
    let result = Ok(DynamicImage::new_rgba8(width, height));
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_crop(ctx: *const WiImage, cx: u32, cy: u32, width: u32, height: u32) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.clone().crop(cx, cy, width, height)),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_color(ctx: *const WiImage) -> *const c_char {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    match &(&*ctx).0 {
        Ok(ref x) => {
            let value: String = match x.color() {
                ::image::ColorType::Gray(bit_depth) => {
                    format!("gray:{}", bit_depth)
                }
                ::image::ColorType::RGB(bit_depth) => {
                    format!("rgb:{}", bit_depth)
                }
                ::image::ColorType::Palette(bit_depth) => {
                    format!("palette:{}", bit_depth)
                }
                ::image::ColorType::GrayA(bit_depth) => {
                    format!("graya:{}", bit_depth)
                }
                ::image::ColorType::RGBA(bit_depth) => {
                    format!("rgba:{}", bit_depth)
                }
                ::image::ColorType::BGR(bit_depth) => {
                    format!("bgr:{}", bit_depth)
                }
                ::image::ColorType::BGRA(bit_depth) => {
                    format!("bgra:{}", bit_depth)
                }
            };
            let value = CString::new(value).expect("CString::new failed");
            value.as_ptr()
        }
        Err(x) => std::ptr::null()
    }
}


#[no_mangle]
pub unsafe extern fn wi_img_grayscale(ctx: *const WiImage) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.grayscale()),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_invert(ctx: *const WiImage) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok({
            let mut x = x.clone();
            x.invert();
            x
        }),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}


#[no_mangle]
pub unsafe extern fn wi_img_resize(ctx: *const WiImage, width: u32, height: u32, format: *const c_char) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if format.is_null() {
        return std::ptr::null_mut();
    }
    let result = CStr::from_ptr(format)
        .to_str()
        .ok()
        .and_then(|format| {
            match format {
                "Nearest" => Some(image::FilterType::Nearest),
                "Triangle" => Some(image::FilterType::Triangle),
                "CatmullRom" => Some(image::FilterType::CatmullRom),
                "Gaussian" => Some(image::FilterType::Gaussian),
                "Lanczos3" => Some(image::FilterType::Lanczos3),
                _ => None
            }
        })
        .ok_or(String::from("invalid filter type"))
        .and_then(|format| {
            match &(&*ctx).0 {
                Ok(ref x) => Ok(x.resize(width, height, format)),
                Err(x) => Err(x.clone()),
            }
        });
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_resize_exact(ctx: *const WiImage, width: u32, height: u32, format: *const c_char) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if format.is_null() {
        return std::ptr::null_mut();
    }
    let result = CStr::from_ptr(format)
        .to_str()
        .ok()
        .and_then(|format| {
            match format.to_lowercase().as_str() {
                "nearest" => Some(image::FilterType::Nearest),
                "triangle" => Some(image::FilterType::Triangle),
                "catmullrom" => Some(image::FilterType::CatmullRom),
                "gaussian" => Some(image::FilterType::Gaussian),
                "lanczos3" => Some(image::FilterType::Lanczos3),
                _ => None
            }
        })
        .ok_or(String::from("invalid filter type"))
        .and_then(|format| {
            match &(&*ctx).0 {
                Ok(ref x) => Ok(x.resize_exact(width, height, format)),
                Err(x) => Err(x.clone()),
            }
        });
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}



#[no_mangle]
pub unsafe extern fn wi_img_thumbnail(ctx: *const WiImage, width: u32, height: u32) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.thumbnail(width, height)),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}


#[no_mangle]
pub unsafe extern fn wi_img_thumbnail_exact(ctx: *const WiImage, width: u32, height: u32) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.thumbnail_exact(width, height)),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_blur(ctx: *const WiImage, sigma: f32) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.blur(sigma)),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_unsharpen(ctx: *const WiImage, sigma: f32, threshold: i32) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.unsharpen(sigma, threshold)),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_filter3x3(ctx: *const WiImage, value: *const f32) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let value = [
        *value.offset(0),
        *value.offset(1),
        *value.offset(2),
        *value.offset(3),
        *value.offset(4),
        *value.offset(5),
        *value.offset(6),
        *value.offset(7),
        *value.offset(8),
    ];
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.filter3x3(&value)),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_adjust_contrast(ctx: *const WiImage, value: f32) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.adjust_contrast(value)),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_brighten(ctx: *const WiImage, value: c_int) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.brighten(value)),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_huerotate(ctx: *const WiImage, value: c_int) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.huerotate(value)),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_flipv(ctx: *const WiImage) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.flipv()),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_fliph(ctx: *const WiImage) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.fliph()),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_rotate90(ctx: *const WiImage) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.rotate90()),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_rotate180(ctx: *const WiImage) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.rotate180()),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_rotate270(ctx: *const WiImage) -> *mut WiImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(ref x) => Ok(x.rotate270()),
        Err(x) => Err(x.clone()),
    };
    let result = Box::new(WiImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_img_save(ctx: *const WiImage, path: *const c_char) -> c_int {
    if ctx.is_null() {
        return 0;
    }
    if path.is_null() {
        return 0;
    }
    let result = CStr::from_ptr(path)
        .to_str()
        .ok()
        .and_then(|path| {
            match &(&*ctx).0 {
                Ok(ref x) => x.save(path).ok(),
                Err(x) => None,
            }
        });
    match result {
        Some(_) => 1,
        None => 0,
    }
}


#[no_mangle]
pub unsafe extern fn wi_img_save_with_format(ctx: *const WiImage, path: *const c_char, format: *const c_char) -> c_int {
    if ctx.is_null() {
        return 0;
    }
    if path.is_null() {
        return 0;
    }
    if format.is_null() {
        return 0;
    }
    let result = CStr::from_ptr(path)
        .to_str()
        .ok()
        .and_then(|path| CStr::from_ptr(format).to_str().ok().map(|x| (path, x)))
        .and_then(|(path, format)| {
            match format {
                "jpeg" => Some((path, image::ImageFormat::JPEG)),
                "png" => Some((path, image::ImageFormat::PNG)),
                _ => None
            }
        })
        .and_then(|(path, format)| {
            match &(&*ctx).0 {
                Ok(ref x) => x.save_with_format(path, format).ok(),
                Err(x) => None,
            }
        });
    match result {
        Some(_) => 1,
        None => 0,
    }
}


///////////////////////////////////////////////////////////////////////////////
// IMAGE-VIEW - HELPER TYPES
///////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct RgbaPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
pub struct LumaPixel {
    pub l: u8
}


///////////////////////////////////////////////////////////////////////////////
// IMAGE-VIEW - METHODS
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_img_width(ctx: *const WiImage) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &(&*ctx).0 {
        Ok(ref x) => {
            x.width() as c_int
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_img_height(ctx: *const WiImage) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &(&*ctx).0 {
        Ok(ref x) => {
            x.height() as c_int
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_width(ctx: *const WiGrayImage) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &(&*ctx).0 {
        Ok(ref x) => {
            x.width() as c_int
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_height(ctx: *const WiGrayImage) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &(&*ctx).0 {
        Ok(ref x) => {
            x.height() as c_int
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_u32_width(ctx: *const WiGrayImageU32) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &(&*ctx).0 {
        Ok(ref x) => {
            x.width() as c_int
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_u32_height(ctx: *const WiGrayImageU32) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &(&*ctx).0 {
        Ok(ref x) => {
            x.height() as c_int
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_img_get_rgba_pixel(ctx: *const WiImage, px: *mut RgbaPixel, cx: u32, cy: u32) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &(&*ctx).0 {
        Ok(ref x) => {
            if x.in_bounds(cx, cy) {
                let [r, g, b, a] = x.get_pixel(cx, cy).0;
                *px = RgbaPixel {r, g, b, a};
                1
            } else {
                -1
            }
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_img_set_rgba_pixel(ctx: *mut WiImage, cx: u32, cy: u32, px: RgbaPixel) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &mut (&mut *ctx).0 {
        Ok(ref mut x) => {
            if x.in_bounds(cx, cy) {
                let value = image::Rgba([px.r, px.g, px.b, px.a]);
                x.put_pixel(cx, cy, value);
                1
            } else {
                -1
            }
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_get_pixel(ctx: *const WiGrayImage, px: *mut u8, cx: u32, cy: u32) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &(&*ctx).0 {
        Ok(ref x) => {
            if x.in_bounds(cx, cy) {
                let [l] = x.get_pixel(cx, cy).0;
                *px = l;
                1
            } else {
                -1
            }
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_set_pixel(ctx: *mut WiGrayImage, cx: u32, cy: u32, px: u8) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &mut (&mut *ctx).0 {
        Ok(ref mut x) => {
            if x.in_bounds(cx, cy) {
                let value = image::Luma([px]);
                x.put_pixel(cx, cy, value);
                1
            } else {
                -1
            }
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_u32_get_pixel(ctx: *const WiGrayImageU32, px: *mut u32, cx: u32, cy: u32) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &(&*ctx).0 {
        Ok(ref x) => {
            if x.in_bounds(cx, cy) {
                let [l] = x.get_pixel(cx, cy).0;
                *px = l;
                1
            } else {
                -1
            }
        }
        Err(x) => -1
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_u32_set_pixel(ctx: *mut WiGrayImage, cx: u32, cy: u32, px: u8) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &mut (&mut *ctx).0 {
        Ok(ref mut x) => {
            if x.in_bounds(cx, cy) {
                let value = image::Luma([px]);
                x.put_pixel(cx, cy, value);
                1
            } else {
                -1
            }
        }
        Err(x) => -1
    }
}

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - CONTRAST
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_grayimg_contrast_adaptive_threshold(
    ctx: *const WiGrayImage,
    block_radius: u32,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::contrast::adaptive_threshold(x, block_radius)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_contrast_equalize_histogram(
    ctx: *const WiGrayImage,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::contrast::equalize_histogram(x)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_contrast_match_histogram(
    ctx: *const WiGrayImage,
    target: *const WiGrayImage,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if target.is_null() {
        return std::ptr::null_mut();
    }
    match (&(&*ctx).0, &(&*target).0) {
        (Ok(x), Ok(y)) => {
            let result = imageproc::contrast::match_histogram(x, y);
            let result = Box::new(WiGrayImage(Ok(result)));
            Box::into_raw(result)
        },
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_contrast_otsu_level(
    ctx: *const WiGrayImage,
) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    match &(&*ctx).0 {
        Ok(x) => imageproc::contrast::otsu_level(x) as c_int,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_contrast_stretch_contrast(
    ctx: *const WiGrayImage,
    lower: u8,
    upper: u8,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::contrast::stretch_contrast(x, lower, upper)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_contrast_threshold(
    ctx: *const WiGrayImage,
    thresh: u8,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::contrast::threshold(x, thresh)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}


///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - CORNERS
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - DISTANCE-TRANSFORM
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_grayimg_distance_transform(
    ctx: *const WiGrayImage,
    norm: *const c_char,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if norm.is_null() {
        return std::ptr::null_mut();
    }
    let value = CStr::from_ptr(norm)
        .to_str()
        .ok()
        .and_then(|x| match x {
            "L1" => Some(imageproc::distance_transform::Norm::L1),
            "LInf" => Some(imageproc::distance_transform::Norm::LInf),
            _ => None,
        });
    if let Some(value) = value {
        let result = match &(&*ctx).0 {
            Ok(x) => Ok(imageproc::distance_transform::distance_transform(x, value)),
            Err(ref x) => Err(x.clone()),
        };
        let result = Box::new(WiGrayImage(result));
        Box::into_raw(result)
    } else {
        std::ptr::null_mut()
    }
}




///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - DRAWING
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - EDGES
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_grayimg_edges_canny(
    ctx: *const WiGrayImage,
    low_threshold: f32, 
    high_threshold: f32,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::edges::canny(x, low_threshold, high_threshold)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - FILTER
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_grayimg_box_filter(
    ctx: *const WiGrayImage,
    x_radius: u32, 
    y_radius: u32,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::filter::box_filter(x, x_radius, y_radius)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_filter3x3(
    ctx: *const WiGrayImage,
    value: *const f32,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let value = [
        *value.offset(0),
        *value.offset(1),
        *value.offset(2),
        *value.offset(3),
        *value.offset(4),
        *value.offset(5),
        *value.offset(6),
        *value.offset(7),
        *value.offset(8),
    ];
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::filter::filter3x3(x, &value)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_filter_gaussian_blur_f32(
    ctx: *const WiGrayImage,
    sigma: f32
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::filter::gaussian_blur_f32(x, sigma)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_filter_median_filter(
    ctx: *const WiGrayImage,
    x_radius: u32, 
    y_radius: u32,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::filter::median_filter(x, x_radius, y_radius)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}


#[no_mangle]
pub unsafe extern fn wi_grayimg_filter_sharpen3x3(
    ctx: *const WiGrayImage,
    low_threshold: f32, 
    high_threshold: f32,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::filter::sharpen3x3(x)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_filter_sharpen_gaussian(
    ctx: *const WiGrayImage,
    sigma: f32,
    amount: f32,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::filter::sharpen_gaussian(x, sigma, amount)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}


///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - GEOMETRIC-TRANSFORMATIONS
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_grayimg_geometric_transformations_translate(
    ctx: *const WiGrayImage,
    t1: i32,
    t2: i32,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::geometric_transformations::translate(x, (t1, t2))),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}



///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - GRADIENTS
///////////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - HAAR
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - HOG
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - HOUGH
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - INTEGRAL-IMAGE
///////////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - LOCAL-BINARY-PATTERNS
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - MAP
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - MATH
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - MORPHOLOGY
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_grayimg_morphology_close(
    ctx: *const WiGrayImage,
    norm: *const c_char,
    k: u8,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if norm.is_null() {
        return std::ptr::null_mut();
    }
    let norm = CStr::from_ptr(norm)
        .to_str()
        .ok()
        .and_then(|x| match x {
            "L1" => Some(imageproc::distance_transform::Norm::L1),
            "LInf" => Some(imageproc::distance_transform::Norm::LInf),
            _ => None,
        });
    if let Some(norm) = norm {
        let result = match &(&*ctx).0 {
            Ok(x) => Ok(imageproc::morphology::close(x, norm, k)),
            Err(ref x) => Err(x.clone()),
        };
        let result = Box::new(WiGrayImage(result));
        Box::into_raw(result)
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_morphology_dilate(
    ctx: *const WiGrayImage,
    norm: *const c_char,
    k: u8,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if norm.is_null() {
        return std::ptr::null_mut();
    }
    let norm = CStr::from_ptr(norm)
        .to_str()
        .ok()
        .and_then(|x| match x {
            "L1" => Some(imageproc::distance_transform::Norm::L1),
            "LInf" => Some(imageproc::distance_transform::Norm::LInf),
            _ => None,
        });
    if let Some(norm) = norm {
        let result = match &(&*ctx).0 {
            Ok(x) => Ok(imageproc::morphology::dilate(x, norm, k)),
            Err(ref x) => Err(x.clone()),
        };
        let result = Box::new(WiGrayImage(result));
        Box::into_raw(result)
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_morphology_erode(
    ctx: *const WiGrayImage,
    norm: *const c_char,
    k: u8,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if norm.is_null() {
        return std::ptr::null_mut();
    }
    let norm = CStr::from_ptr(norm)
        .to_str()
        .ok()
        .and_then(|x| match x {
            "L1" => Some(imageproc::distance_transform::Norm::L1),
            "LInf" => Some(imageproc::distance_transform::Norm::LInf),
            _ => None,
        });
    if let Some(norm) = norm {
        let result = match &(&*ctx).0 {
            Ok(x) => Ok(imageproc::morphology::erode(x, norm, k)),
            Err(ref x) => Err(x.clone()),
        };
        let result = Box::new(WiGrayImage(result));
        Box::into_raw(result)
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_morphology_open(
    ctx: *const WiGrayImage,
    norm: *const c_char,
    k: u8,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if norm.is_null() {
        return std::ptr::null_mut();
    }
    let norm = CStr::from_ptr(norm)
        .to_str()
        .ok()
        .and_then(|x| match x {
            "L1" => Some(imageproc::distance_transform::Norm::L1),
            "LInf" => Some(imageproc::distance_transform::Norm::LInf),
            _ => None,
        });
    if let Some(norm) = norm {
        let result = match &(&*ctx).0 {
            Ok(x) => Ok(imageproc::morphology::open(x, norm, k)),
            Err(ref x) => Err(x.clone()),
        };
        let result = Box::new(WiGrayImage(result));
        Box::into_raw(result)
    } else {
        std::ptr::null_mut()
    }
}



///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - NOISE
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_grayimg_gaussian_noise(
    ctx: *const WiGrayImage,
    mean: f64, 
    stddev: f64, 
    seed: u64,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::noise::gaussian_noise(x, mean, stddev, seed)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

#[no_mangle]
pub unsafe extern fn wi_grayimg_salt_and_pepper_noise(
    ctx: *const WiGrayImage,
    rate: f64, 
    seed: u64,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::noise::salt_and_pepper_noise(x, rate, seed)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}


///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - PIXELOPS
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - PROPERTY-TESTING
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - RECT
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - REGION-LABELLING
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_grayimg_region_labelling_connected_components(
    ctx: *const WiGrayImage,
    conn: *const c_char,
    background: u8,
) -> *mut WiGrayImageU32 {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    if conn.is_null() {
        return std::ptr::null_mut();
    }
    let conn = CStr::from_ptr(conn)
        .to_str()
        .ok()
        .and_then(|x| match x {
            "four" => Some(imageproc::region_labelling::Connectivity::Four),
            "eight" => Some(imageproc::region_labelling::Connectivity::Eight),
            _ => None,
        });
    if let Some(conn) = conn {
        let bg = image::Luma([background]);
        let result = match &(&*ctx).0 {
            Ok(x) => Ok(imageproc::region_labelling::connected_components(x, conn, bg)),
            Err(ref x) => Err(x.clone()),
        };
        let result = Box::new(WiGrayImageU32(result));
        Box::into_raw(result)
    } else {
        std::ptr::null_mut()
    }
}



///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - SEAM-CARVING
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn wi_grayimg_seam_carving_shrink_width(
    ctx: *const WiGrayImage,
    target_width: u32,
) -> *mut WiGrayImage {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let result = match &(&*ctx).0 {
        Ok(x) => Ok(imageproc::seam_carving::shrink_width(x, target_width)),
        Err(ref x) => Err(x.clone()),
    };
    let result = Box::new(WiGrayImage(result));
    Box::into_raw(result)
}

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - STATS
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - SUPPRESS
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - TEMPLATE-MATCHING
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - UNION-FIND
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// IMAGEPROC - UTILS
///////////////////////////////////////////////////////////////////////////////



