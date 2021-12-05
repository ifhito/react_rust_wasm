extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;
use imageproc::drawing::draw_text_mut;
use image::*;
use js_sys::*;
use rusttype::{Font, Scale};
use rand::seq::SliceRandom;
use rand::Rng;
// use imageproc::drawing::draw_text_mut;
// use rusttype::{Font, Scale};
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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
    console_log!("{:?}", img);
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
#[wasm_bindgen]
pub fn create_glitch(arr: Uint8Array, fmt: &str) -> Uint8Array{
    console_error_panic_hook::set_once();
    // glitchの感覚をランダムで選択するための初期値
    let glitch_list:Vec<u32> = vec![5,10,15,20,15,30];
    let glitch_list_i:Vec<u32> = vec![1,5,7,10,15,20];
    // 色を変更するかどうかはランダムで決定する
    let glitch_boolean:Vec<bool> = vec![true, false];
    // Javascriptから上がってきた行列を入れるバッファー
    let buffer: Vec<u8> = arr.to_vec();
    let mut img = load_from_memory(&buffer).expect("Error occurs at load image from buffer.");
    let (width, height) = img.dimensions();
    let mut flag = false;
    let mut rng = rand::thread_rng();
    let mut choice = glitch_list.choose(&mut rng).unwrap();
    let mut i= *glitch_list_i.choose(&mut rng).unwrap();
    //全てのピクセルに処理を行う
    for y in 0..height {
        // 10が選択されたら10の倍数でglitchをする
        if y % choice == 0 {
            // 次に選択された倍数になる高さではflagは再度falseになり、その次の倍数の高さでずれる分の新しいglitchが作られる。
            if flag {
                flag = false;
                choice = glitch_list.choose(&mut rng).unwrap();
                i= *glitch_list_i.choose(&mut rng).unwrap()
            }else {
                //flagをtrueにしてpixelをずらす
                flag = true;
            }
        }
        // 全ての横幅のpixel分処理する
        for x in 0..width {
            let mut to_pixel;
            // flagがtrueだった場合はi個先のpixelにずらす
            if flag == false {
                to_pixel = img.get_pixel(x, y);
            }else{
                to_pixel = img.get_pixel(i, y);
            }
            // ランダムで色を変更するかが決まる。trueだったらそこの部分の色をランダムで変更する
            if *glitch_boolean.choose(&mut  rng).unwrap() == true{
                // let red = to_pixel[0];
                // let green = to_pixel[1];
                // let blue = to_pixel[2];
                let alpha = to_pixel[3];
                let new_color = [rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), alpha];
                to_pixel = Rgba(new_color);
            }
            img.put_pixel(x, y, to_pixel);
            //ずらし先のiはwidthを超えるとエラーになるので越えた段階でi=0とする
            if i >= width - 1{
                i = 0;
            }else {
                i += 1;
            }
        }
    }
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