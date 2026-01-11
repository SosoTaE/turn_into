use image::{Rgba, RgbaImage};
use kmeans_colors::{get_kmeans, Kmeans, Calculate};
use palette::{FromColor, Lab, Srgb};
use rayon::prelude::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <source_img> <style_img> <output_path>", args[0]);
        process::exit(1);
    }

    let k = 8; // Number of clusters

    println!("Step 1: Extracting Frequency-Sorted Palette from Source...");
    let (img_a, palette_a) = get_image_and_sorted_palette(&args[1], k);

    println!("Step 2: Extracting Frequency-Sorted Palette from Style...");
    let (_, palette_b) = get_image_and_sorted_palette(&args[2], k);

    // Step 3: Frequency Mapping
    // Instead of finding 'nearest' color, we map index-to-index.
    // Most common color in A maps to most common color in B.
    let color_map = palette_b;

    println!("Step 4: Transforming image using frequency mapping...");
    let result_img = transform_image_parallel(&img_a, &palette_a, &color_map);

    result_img.save(&args[3]).expect("Failed to save result");
    println!("Success! Result saved to {}", args[3]);
}

/// Loads an image, runs K-Means, and returns (Full Image, Palette sorted by prevalence)
fn get_image_and_sorted_palette(path: &str, k: usize) -> (RgbaImage, Vec<Lab>) {
    let img = image::open(path)
        .unwrap_or_else(|_| {
            eprintln!("Error: Could not find image at {}", path);
            process::exit(1);
        })
        .to_rgba8();

    // Resize for analysis to speed up K-Means
    let thumb = image::imageops::thumbnail(&img, 128, 128);
    let labs: Vec<Lab> = thumb.pixels().map(|p| rgb_to_lab(*p)).collect();

    // Run K-Means
    let result = get_kmeans(k, 20, 5.0, false, &labs, 0);

    // Associate centroids with their weights (pixel counts)
    let mut palette_with_counts: Vec<(Lab, usize)> = result.centroids
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            // Count how many pixels were assigned to this specific centroid
            let count = result.indices.iter().filter(|&&idx| idx == i as u8).count();
            (c, count)
        })
        .collect();

    // Sort by count descending (most common colors first)
    palette_with_counts.sort_by(|a, b| b.1.cmp(&a.1));

    // Return just the colors in their new sorted order
    let sorted_palette = palette_with_counts.into_iter().map(|(c, _)| c).collect();
    (img, sorted_palette)
}

/// Parallel pixel transformation using index-based mapping
fn transform_image_parallel(img: &RgbaImage, source_palette: &[Lab], map: &[Lab]) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut output = RgbaImage::new(width, height);

    output.enumerate_pixels_mut().par_bridge().for_each(|(x, y, out_pixel)| {
        let original_pixel = img.get_pixel(x, y);
        let lab_pixel = rgb_to_lab(*original_pixel);

        // Find which palette index the current pixel belongs to
        let mut best_idx = 0;
        let mut min_dist = f32::MAX;
        for (i, source_color) in source_palette.iter().enumerate() {
            let d = delta_e_sq(&lab_pixel, source_color);
            if d < min_dist {
                min_dist = d;
                best_idx = i;
            }
        }

        // Map the index directly to the target palette
        let mapped_lab = map[best_idx];
        let final_rgb = Srgb::from_color(mapped_lab);

        *out_pixel = Rgba([
            (final_rgb.red * 255.0).round() as u8,
            (final_rgb.green * 255.0).round() as u8,
            (final_rgb.blue * 255.0).round() as u8,
            original_pixel[3], // Keep original alpha
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