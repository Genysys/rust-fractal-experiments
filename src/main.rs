extern crate piston_window;
extern crate piston;
extern crate num;
extern crate image;

use image::{ImageBuffer, Rgba};
use piston_window::{PistonWindow, WindowSettings, Texture, TextureSettings, Image};
use piston::input::*;
use num::complex::Complex;

fn hue_to_rgb(i: f32) -> f32 {
    let mut h = i as f32;
    if h > 1.0 {
        h = h - 1.0;
    }
    let ret = match h {
        h if h < 0.16 => 6.0 * h,
        h if h < 0.5 => 1.0,
        h if h < 0.67 => (0.67 - h) * 6.0,
        _ => 1.0
    };
    ret * 255.0
}

fn julia(imgx: u32, imgy: u32, offsetx: f32, offsety: f32, zoom: f32, c: Complex<f32>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let max_iterations = 255u16;
    let scalex = zoom / imgx as f32;
    let scaley = zoom / imgy as f32;

    ImageBuffer::from_fn(imgx, imgy, |x, y| {
        let cy = (y as f32 + offsety) * scaley - (zoom / 2.0);
        let cx = (x as f32 + offsetx) * scalex - (zoom / 2.0);

        let mut z = Complex::new(cx, cy);

        let mut i = 0;

        for t in 0..max_iterations {
            if z.norm() > 2.0 {
                break
            }
            z = z * z + c;
            i = t;
        }
        let lum = i as f32;

        image::Rgba([hue_to_rgb(lum/255.0 + 0.3) as u8,
                    hue_to_rgb(lum/255.0) as u8,
                    hue_to_rgb(lum/255.0 - 0.3) as u8,
                    255])
    })
}

fn main() {
    let imgx = 512;
    let imgy = 512;
    let window: PistonWindow =
        WindowSettings::new("Julia set", [imgx, imgy])
        .build().unwrap();
    let factory = &mut *window.factory.borrow_mut();

    // Create a new ImgBuf with width: imgx and height: imgy
    let zoom = 4.0;
    let offsetx = 0.0;
    let offsety = 0.0;
    let mut c_param = Complex::new(0.0, 0.0);
    let imgbuf = julia(imgx, imgy, offsetx, offsety, zoom, c_param);
    let mut px = Texture::from_image(factory, &imgbuf, &TextureSettings::new()).unwrap();
    let img = Image::new();
    let mut counter = 2;

    for e in window.clone() {
        print!("Frame {} {} \r", counter, zoom);
        e.mouse_cursor(|x, y| {
            // Truncate mouse values to inside window and normalize
            let c_re = 2.0 * ((x as f32 - (imgx/2) as f32)) / (imgx as f32);
            let c_im = 2.0 * ((y as f32 - (imgy/2) as f32)) / (imgy as f32);
            c_param = Complex::new(c_re, c_im);
        });
        e.draw_2d(|c, g| {
            let imgbuf = julia(imgx, imgy, offsetx, offsety, zoom, c_param);
            px.update(factory, &imgbuf).unwrap();
            img.draw(&px, &c.draw_state, c.transform, g)
        });
        counter = counter + 1;
    }
}
