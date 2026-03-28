use image::{Rgba, RgbaImage};
use kmeans_colors::get_kmeans;
use palette::{FromColor, Lab, Srgb};
use rayon::prelude::*;
use std::env;
use std::io;
use std::process;
use dashmap::DashSet;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <source> <style> <output> [max_k]", args[0]);
        process::exit(1);
    }

    let src = &args[1];
    let style = &args[2];
    let out = &args[3];
    let max_k: usize = args.get(4).and_then(|v| v.parse().ok()).unwrap_or(8);

    run_photo_mode(src, style, out, max_k);

    Ok(())
}

fn count_unique_colors(img: &RgbaImage) -> usize {
    println!("🔍 Analyzing {} pixels for unique colors...", img.width() * img.height());

    let unique_colors = DashSet::new();

    img.pixels().par_bridge().for_each(|pixel| {
        unique_colors.insert(pixel.0);
    });

    unique_colors.len()
}

fn run_photo_mode(src: &str, style: &str, out: &str, k: usize) {
    println!("Step 1: Analyzing Palettes...");
    let (img_a, pal_a) = get_image_and_sorted_palette(src, k);
    let color_count = count_unique_colors(&img_a);
    println!("✓ Source image contains {} unique colors.", color_count);
    
    let (_, pal_b) = get_image_and_sorted_palette(style, k);

    println!("Step 2: Processing Pixels in Parallel...");
    let result = transform_image_parallel(&img_a, &pal_a, &pal_b);

    result.save(out).expect("Failed to save result");
    println!("✓ Photo saved to {}", out);
}

fn get_image_and_sorted_palette(path: &str, k: usize) -> (RgbaImage, Vec<Lab>) {
    let img = image::open(path).expect("File error").to_rgba8();
    let thumb = image::imageops::thumbnail(&img, 128, 128);
    let labs: Vec<Lab> = thumb.pixels().map(|p| rgb_to_lab(*p)).collect();

    let result = get_kmeans(k, 20, 5.0, false, &labs, 0);

    let mut pal: Vec<(Lab, usize)> = result.centroids.iter().enumerate()
        .map(|(i, &c)| (c, result.indices.iter().filter(|&&idx| idx == i as u8).count()))
        .collect();

    pal.sort_by(|a, b| b.1.cmp(&a.1));
    (img, pal.into_iter().map(|(c, _)| c).collect())
}

fn transform_image_parallel(img: &RgbaImage, src_pal: &[Lab], tgt_pal: &[Lab]) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut output = RgbaImage::new(width, height);

    output.enumerate_pixels_mut().par_bridge().for_each(|(x, y, out_pixel)| {
        let lab_px = rgb_to_lab(*img.get_pixel(x, y));
        let mut best_idx = 0;
        let mut min_dist = f32::MAX;

        for (i, src_color) in src_pal.iter().enumerate() {
            let d = delta_e_sq(&lab_px, src_color);
            if d < min_dist { 
                min_dist = d; 
                best_idx = i; 
            }
        }

        let final_rgb = Srgb::from_color(tgt_pal[best_idx]);
        *out_pixel = Rgba([
            (final_rgb.red * 255.0) as u8,
            (final_rgb.green * 255.0) as u8,
            (final_rgb.blue * 255.0) as u8,
            255
        ]);
    });
    output
}

fn rgb_to_lab(p: Rgba<u8>) -> Lab {
    let srgb = Srgb::new(p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0);
    Lab::from_color(srgb)
}

fn delta_e_sq(c1: &Lab, c2: &Lab) -> f32 {
    (c1.l - c2.l).powi(2) + (c1.a - c2.a).powi(2) + (c1.b - c2.b).powi(2)
}
