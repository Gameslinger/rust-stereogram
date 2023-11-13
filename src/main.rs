use clap::Parser;
use image::{Rgb, RgbImage};
use rand::{thread_rng, Rng};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Image file name that represent depth distribution
    #[arg(short, long)]
    depth_source_image_file: String,

    /// Size of repeating dots pattern square (in pixels)
    #[arg(short, long, default_value_t = 40)]
    pattern_size: u32,

    /// Distortion to represent depth. 0.1 - flatest, 0.9 - deepest
    #[arg(short, long, default_value_t = 0.6)]
    shift_amplitude: f64,

    /// Width of outer image (in pixels)
    #[arg(short('W'), long)]
    width: Option<u32>,

    /// Height of outer image (in pixels)
    #[arg(short('H'), long)]
    height: Option<u32>,

    /// Stereogram output image file name
    #[arg(short, long)]
    out_image_file: String,
}

fn main() {
    let args: Args = Args::parse();

    let depthmap = image::open(args.depth_source_image_file.trim())
        .expect("Unable to open depth map!")
        .to_luma8();
    let image_width: u32 = args.width.unwrap_or(depthmap.width());
    let image_height: u32 = args.height.unwrap_or(depthmap.height());
    let pix_width: u32 = args.pattern_size;
    let shift_amplitude: f64 = args.shift_amplitude;
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
                let min_width = out_image.width().min(depthmap.width());
                let min_height = out_image.height().min(depthmap.height());
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
    out_image
        .save(args.out_image_file)
        .expect("Can't save output file");
}
