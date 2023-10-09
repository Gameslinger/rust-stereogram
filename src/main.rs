use image::{Rgb, RgbImage};
use rand::{thread_rng, Rng};
use std::env;

//Usage: [out width] [out height] [pattern_width] [shift_amplitude] [depthmap]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        panic!("Invalid parameters!");
    }
    let image_width: u32 = args[1].trim().parse().expect("Invalid image width");
    let image_height: u32 = args[2].trim().parse().expect("Invalid image height");
    let pix_width: u32 = args[3].trim().parse().expect("Invalid pattern width");
    let shift_amplitude: f64 = args[4]
        .trim()
        .parse()
        .expect("Invalid shift shift_amplitude");
    let depthmap = image::open(args[5].trim())
        .expect("Unable to open depth map!")
        .to_luma8();
    //Generate random pixels
    let mut src_buf: Vec<Vec<f64>> = vec![vec![0.0; pix_width as usize]; image_height as usize];
    let mut rng = thread_rng();
    for i in 0..image_height {
        for j in 0..pix_width {
            src_buf[i as usize][j as usize] = rng.gen_range(0..64) as f64 / 64.;
            //rng.gen_range(0.0..=1.0);
        }
    }
    //Load depth map
    let mut brightest: u8 = 0;
    let mut darkest: u8 = 255;
    for i in 0..depthmap.height() {
        for j in 0..depthmap.width() {
            let pix = depthmap.get_pixel(j, i).0[0];
            if pix > brightest {
                brightest = pix;
            }
            if pix < darkest {
                darkest = pix;
            }
        }
    }
    let brightest: f64 = brightest as f64;
    let darkest: f64 = darkest as f64;
    //Allocate picture
    let mut out_image = RgbImage::new(image_width, image_height);
    //Shift pixels by depth map
    for r in 0..image_height {
        for c in 0..image_width {
            if c <= pix_width {
                let intensity =
                    (src_buf[(r % pix_width) as usize][(c % pix_width) as usize] * 255.) as u8;
                out_image.put_pixel(c, r, Rgb([intensity, intensity, intensity]));
            } else {
                let min_width = if out_image.width() < depthmap.width() {
                    out_image.width()
                } else {
                    depthmap.width()
                };
                let min_height = if out_image.height() < depthmap.height() {
                    out_image.height()
                } else {
                    depthmap.height()
                };
                let intensity = (((depthmap.get_pixel(c % min_width, r % min_height))[0] as f64)
                    - darkest)
                    / (brightest - darkest);
                let offset = (pix_width as f64) * shift_amplitude * intensity; //(((r as f64).sin() + 1.) / 2.)) as u32;
                let offset_pixel1 = *out_image.get_pixel(c - pix_width + (offset as u32), r);
                out_image.put_pixel(c, r, offset_pixel1);
                // let offset_pixel2 = *out_image.get_pixel(c - pix_width + (offset as u32), r);
                // let decimal = offset - offset.floor();
                // let avg_int = (offset_pixel1.0[0] as f64 * (1. - decimal)
                //     + offset_pixel2.0[0] as f64 * decimal) as u8;
                // out_image.put_pixel(c, r, Rgb([avg_int, avg_int, avg_int]));
            }
        }
    }
    //Save image
    out_image.save("stereogram.png").unwrap();
}
