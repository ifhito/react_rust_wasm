extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;
use imageproc::drawing::draw_text_mut;
use image::*;
use js_sys::*;
use rusttype::{Font, Scale};
use rand::seq::SliceRandom;
use rand::Rng;
use web_sys::console;
// use imageproc::drawing::draw_text_mut;
// use rusttype::{Font, Scale};
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(s: u32);
}
// macro_rules! console_log {
//     // Note that this is using the `log` function imported above during
//     // `bare_bones`
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

// #[wasm_bindgen]
// pub fn resize_image(arr: Uint8Array, width: usize, height:usize, fmt: &str) -> Uint8Array {
//     console_error_panic_hook::set_once();
//     let buffer = arr.to_vec();
//     console_log!("{}", fmt);
//     let mut img = load_from_memory(&buffer).expect("Error occurs at load image from buffer.");
//     let font = Vec::from(include_bytes!("../assets/image/DelaGothicOne-Regular.ttf") as &[u8]);
//     let font = Font::try_from_vec(font).unwrap();
//     let scale = Scale {
//         x: scale_height,
//         y: scale_height,
//     };
//     draw_text_mut(&mut img, Rgba([255u8, 255u8, 255u8, 255u8]), 50, 50, scale, &font, text);
//     // let resized = img.unsharpen(width as f32, height as i32);
//     // let resized = resized.huerotate(height as i32);
//     let result = save_to_buffer(img, fmt);

//     Uint8Array::new(&unsafe { Uint8Array::view(&result)}.into())

// }
#[wasm_bindgen]
pub fn add_font_image(arr: Uint8Array, scale_height: f32, pixels_from_up: u32, pixels_from_left: u32, text: &str, fmt: &str) -> Uint8Array{
    let buffer = arr.to_vec();
    let mut img = load_from_memory(&buffer).expect("Error occurs at load image from buffer.");
    let font = Vec::from(include_bytes!("../assets/image/DelaGothicOne-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    let scale = Scale {
        x: scale_height,
        y: scale_height,
    };
    draw_text_mut(&mut img, Rgba([255u8, 255u8, 255u8, 255u8]), pixels_from_left, pixels_from_up, scale, &font, text);
    let result = save_to_buffer(img, fmt);

    Uint8Array::new(&unsafe { Uint8Array::view(&result)}.into())
}
fn random_color_choice(red:u8, green:u8, blue:u8, alpha:u8) -> image::Rgba<u8>{
    let mut rng = rand::thread_rng();
    if [true, false].choose(&mut  rng).unwrap() == &true {
        Rgba([rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), alpha])
    }else{
        Rgba([red, green, blue, alpha])
    }
}
#[wasm_bindgen]
pub fn create_glitch(arr: Uint8Array, fmt: &str) -> Uint8Array{
    console_error_panic_hook::set_once();
    // glitchの感覚をランダムで選択するための初期値
    let glitch_list_x:Vec<u32> = vec![1,5,7,10,15,20];
    let glitch_list_y:Vec<u32> = vec![5,10,15,20,15,30];
    // 色を変更するかどうかはランダムで決定する
    // let glitch_boolean:Vec<bool> = vec![true, false];
    // Javascriptから上がってきた行列を入れるバッファー
    let buffer: Vec<u8> = arr.to_vec();
    let mut img = load_from_memory(&buffer).expect("Error occurs at load image from buffer.");
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();
    let mut choice = glitch_list_y.choose(&mut rng).unwrap();
    let mut n = 0;
    while n < height{
        if n % choice != 0 {
            let mut i = *glitch_list_x.choose(&mut rng).unwrap();
            for _t in 0..*choice{
                console::log_1(&JsValue::from(n+_t));
                if n+_t >= height -1 {break};
                for x in 0..width{
                    let mut to_pixel = img.get_pixel(i, n+_t);
                    to_pixel = random_color_choice(to_pixel[0], to_pixel[1], to_pixel[2], to_pixel[3]);
                    img.put_pixel(x, n+_t, to_pixel);
                    i = if i >= width -1 { 0 } else { i+1 };
                }
            }
            n += choice;
            choice = glitch_list_y.choose(&mut rng).unwrap();
        } else {
            for x in 0..width{
                let mut to_pixel = img.get_pixel(x, n);
                to_pixel = random_color_choice(to_pixel[0], to_pixel[1], to_pixel[2], to_pixel[3]);
                img.put_pixel(x, n, to_pixel);
            }
            n += 1;
        }
}
    // for y in 0..height {
    //     for x in 0..width {
    //         let mut to_pixel = img.get_pixel(x, y);
    //         if glitch_boolean.choose(&mut  rng).unwrap() == &true {
    //             let alpha = to_pixel[3];
    //             let new_color = [rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), alpha];
    //             to_pixel = Rgba(new_color);
    //         }
    //         img.put_pixel(x, y, to_pixel);
    //     }
    // }
    let result = save_to_buffer(img, fmt);

    Uint8Array::new(&unsafe { Uint8Array::view(&result)}.into())
}

fn save_to_buffer(img: DynamicImage, fmt_str:&str) -> Vec<u8> {
    console_error_panic_hook::set_once();

    let fmt = match fmt_str {
        "png" => ImageOutputFormat::Png,
        "gif" => ImageOutputFormat::Gif,
        "bmp" => ImageOutputFormat::Bmp,
        "jpg" => ImageOutputFormat::Jpeg(80),
        unsupport => ImageOutputFormat::Unsupported(String::from(unsupport)),
    };

    let mut result: Vec<u8> = Vec::new();
    img.write_to(&mut result, fmt).expect("Error occurs at save image from buffer.");

    result
}


// pub fn test(a:u32) -> u32{
//     let mut image = image::open("assets/image/whilte.jpg").unwrap();
//     let font = Vec::from(include_bytes!("../assets/image/DelaGothicOne-Regular.ttf") as &[u8]);
//     let font = Font::try_from_vec(font).unwrap();

//     let height = 300.0;
//     let scale = Scale {
//         x: height,
//         y: height,
//     };

//     let text = "test";
//     draw_text_mut(&mut image, Rgba([255u8, 255u8, 255u8, 255u8]), 50, 0, scale, &font, text);
//     image.save("test.jpg").unwrap();
//     return a;

// }

// #[wasm_bindgen]
// pub fn fib(n: u32) -> u32 {
//     if n == 0 || n == 1 {
//         return n;
//     }

//     fib(n - 1) + fib(n - 2)
// }