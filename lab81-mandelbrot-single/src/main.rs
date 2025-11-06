use hsv_to_rgb::hsv_to_rgb;
use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use std::time::Instant;

fn main() {
    let image_width: u32 = 1920;
    let image_height: u32 = 1080;
    let max_iterations: u32 = 1000;

    let mut imgbuf = ImageBuffer::new(image_width, image_height);

    let x_min: f64 = -2.0;
    let x_max: f64 = 1.0;
    let y_min: f64 = -1.0;
    let y_max: f64 = 1.0;

    let start = Instant::now();
    for y in 0..image_height {
        for x in 0..image_width {
            // Map pixel coordinates to complex plane
            let cx = x_min + (x as f64 / image_width as f64) * (x_max - x_min);
            let cy = y_min + (y as f64 / image_height as f64) * (y_max - y_min);
            let c = Complex::new(cx, cy);

            // Initial value of z is 0 + 0i
            let mut z = Complex::new(0.0, 0.0);
            let mut n = 0;

            // Iterate until escape or max iterations
            while n < max_iterations && z.norm_sqr() <= 4.0 {
                z = z * z + c;
                n += 1;
            }

            // Color the pixel based on iteration count
            let pixel = if n == max_iterations {
                // Point is in the set - color it black
                Rgb([0, 0, 0])
            } else {
                // Point escaped - use smooth coloring based on iteration count
                let hue = (n as f32 / max_iterations as f32) * 360.0;
                hsv_to_rgb(hue, 1.0, 1.0)
            };
            imgbuf.put_pixel(x, y, pixel);
        }
    }

    let duration = start.elapsed();
    println!("Rendering time: {:?}", duration);

    std::fs::create_dir_all("./out").unwrap();
    imgbuf.save("./out/mandelbrot_single.png").unwrap();
    println!("Image saved to ./out/mandelbrot_single.png");
}
