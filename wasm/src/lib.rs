extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;
use image::*;
use js_sys::*;
use rand::seq::SliceRandom;
use rand::Rng;
// use web_sys::console;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(s: u32);
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
    // Javascriptから上がってきた行列を入れるバッファー
    let buffer: Vec<u8> = arr.to_vec();
    let mut img = load_from_memory(&buffer).expect("Error occurs at load image from buffer.");
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();
    let mut choice = glitch_list_y.choose(&mut rng).unwrap();
    let mut boolean_glitch = [true, false].choose(&mut rng).unwrap();
    let mut n = 0;
    while n < height{
        let mut i = 0;
        if boolean_glitch == &true {i = *glitch_list_x.choose(&mut rng).unwrap();}
        let mut t = 0;
        while &t < choice && n+t < height {
            // console::log_1(&JsValue::from(n+t));
            for x in 0..width{
                let mut to_pixel = img.get_pixel(i, n+t);
                to_pixel = random_color_choice(to_pixel[0], to_pixel[1], to_pixel[2], to_pixel[3]);
                img.put_pixel(x, n+t, to_pixel);
                i = if i >= width -1 { 0 } else { i+1 };
            }
            t += 1;
        }
        n += t;
        choice = glitch_list_y.choose(&mut rng).unwrap();
        boolean_glitch = [true, false].choose(&mut rng).unwrap();
    }
    let result = saveto_buffer(img, fmt);

    Uint8Array::new(&unsafe { Uint8Array::view(&result)}.into())
}

fn saveto_buffer(img: DynamicImage, fmt_str:&str) -> Vec<u8> {
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
