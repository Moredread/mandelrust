//! Simple Mandelbrot image generator
//!
//! This generates a Mandelbrot set image and saves it to a file.
//! The old interactive GUI version using Piston has been removed as those
//! dependencies are no longer maintained. See src/main.rs.old for the old version.

use mandelrust::mandelbrot::*;
use rug::Float;

fn main() {
    println!("Generating Mandelbrot set image...");

    let prec = 128u32; // Precision in bits
    let width = 1920u32;
    let height = 1080u32;
    let max_iterations = 1000u32;

    // Center on the classic Mandelbrot view
    let center_x = Float::with_val(prec, -0.5);
    let center_y = Float::with_val(prec, 0.0);
    let zoom = Float::with_val(prec, 1.0);

    println!("Canvas: {}x{}", width, height);
    println!("Max iterations: {}", max_iterations);
    println!("Precision: {} bits", prec);

    let canvas = CanvasSize::new_from_center(width, height, [center_x, center_y], zoom);

    println!("Calculating Mandelbrot set...");
    let data = calculate_all_mpfr(canvas.clone(), max_iterations);

    println!("Generating image...");
    let img = make_image(data, canvas, max_iterations);

    let output_path = "mandelbrot.png";
    println!("Saving to {}...", output_path);
    img.save(output_path).expect("Failed to save image");

    println!("Done! Image saved to {}", output_path);
}
