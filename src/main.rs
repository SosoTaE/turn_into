use image::{Rgba, RgbaImage};
use kmeans_colors::{get_kmeans, Calculate};
use palette::{FromColor, Lab, Srgb};
use rayon::prelude::*;
use std::env;
use std::process;

fn main() {
    // 1. Collect arguments
    let args: Vec<String> = env::args().collect();

    // Usage: cargo run -- <path_a> <path_b> <output_path>
    if args.len() < 4 {
        eprintln!("Usage: {} <image_a_path> <image_b_path> <output_path>", args[0]);
        process::exit(1);
    }

    let path_a = &args[1];
    let path_b = &args[2];
    let path_out = &args[3];

    let k = 8;

    // 2. Load and Extract Palettes
    println!("Extracting palette from Image A: {}...", path_a);
    let (img_a, palette_a) = get_image_and_palette(path_a, k);

    println!("Extracting palette from Image B: {}...", path_b);
    let (_, palette_b) = get_image_and_palette(path_b, k);

    // 3. Create Color Mapping (A -> B)
    let color_map: Vec<Lab> = palette_a
        .iter()
        .map(|color_a| {
            palette_b
                .iter()
                .min_by(|&&b1, &&b2| {
                    let d1 = delta_e_sq(color_a, &b1);
                    let d2 = delta_e_sq(color_a, &b2);
                    d1.partial_cmp(&d2).unwrap()
                })
                .unwrap()
                .clone()
        })
        .collect();

    // 4. Transform Image A using the palette of B
    println!("Applying color transformation...");
    let result_img = transform_image_parallel(&img_a, &palette_a, &color_map);

    // 5. Save output
    if let Err(e) = result_img.save(path_out) {
        eprintln!("Failed to save output image: {}", e);
        process::exit(1);
    }

    println!("Success! Saved to: {}", path_out);
}

// --- Logic Functions ---

fn get_image_and_palette(path: &str, k: usize) -> (RgbaImage, Vec<Lab>) {
    let img = image::open(path)
        .unwrap_or_else(|e| {
            eprintln!("Error opening {}: {}", path, e);
            process::exit(1);
        })
        .to_rgba8();

    // Optimization: Thumbnails for fast K-Means
    let thumb = image::imageops::thumbnail(&img, 128, 128);
    let labs: Vec<Lab> = thumb.pixels().map(|p| rgb_to_lab(*p)).collect();

    let result = get_kmeans(k, 20, 5.0, false, &labs, 0);
    (img, result.centroids)
}

fn transform_image_parallel(img: &RgbaImage, source_palette: &[Lab], map: &[Lab]) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut output = RgbaImage::new(width, height);

    // Using Rayon bridge to parallelize the pixel buffer mutation
    output.enumerate_pixels_mut().par_bridge().for_each(|(x, y, out_pixel)| {
        let original_pixel = img.get_pixel(x, y);
        let lab_pixel = rgb_to_lab(*original_pixel);

        let mut best_idx = 0;
        let mut min_dist = f32::MAX;
        for (i, source_color) in source_palette.iter().enumerate() {
            let d = delta_e_sq(&lab_pixel, source_color);
            if d < min_dist {
                min_dist = d;
                best_idx = i;
            }
        }

        let mapped_lab = map[best_idx];
        let final_rgb = Srgb::from_color(mapped_lab);

        *out_pixel = Rgba([
            (final_rgb.red * 255.0).round() as u8,
            (final_rgb.green * 255.0).round() as u8,
            (final_rgb.blue * 255.0).round() as u8,
            original_pixel[3],
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