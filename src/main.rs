extern crate image;
use std::error::Error;
extern crate colorous;
use colorous::Color;
use std::f32::consts::PI;
use std::time::Instant;

fn step(
    width: usize,
    height: usize,
    wavepropagationfactor: f32,
    past: &Vec<f32>,
    present: &Vec<f32>,
    future: &mut Vec<f32>,
) {
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let index: usize = x + y * width;
            // Ensure future is zero
            future[index] = 0.0;
            // Check to avoid overflow at boundary box:
            if x >= 1 && x < width && y >= 1 && y < height {
                future[index] += present[index - 1];
                future[index] += present[index + 1];
                future[index] += present[index - width];
                future[index] += present[index + width];
                future[index] -= 4.0 * present[index];
                future[index] *= wavepropagationfactor;

                future[index] += 2.0 * present[index];
                future[index] -= past[index];
            }
        }
    }
}

fn physics() -> (usize, usize, f32, f32, f32) {
    // SI units preferred
    let temperature_degc = 25.0; // Degree Celsius
    let airspeed_mps = 331.3 * (1.0 + temperature_degc / 273.15 as f32).sqrt(); // m/s
    let xdim_m = 0.10; // m
    let ydim_m = 0.10; // m
    let resolution_m = 0.0002; // m
    let safety_factor = 0.5; // To ensure time resolution is small enough, otherwise not stable.
    let dt_s = resolution_m / airspeed_mps * safety_factor; // s
    let x_steps = xdim_m / resolution_m; // steps
    let y_steps = ydim_m / resolution_m; // steps
    let dx_m = xdim_m / x_steps; // m
                                 //    let dy_m = ydim_m/height; // m
    let wavepropagationfactor = dt_s * dt_s * airspeed_mps * airspeed_mps / (dx_m * dx_m);

    let width: usize = x_steps as usize;
    let height: usize = y_steps as usize;

    return (width, height, wavepropagationfactor, dt_s, resolution_m);
}

fn sound_source(resolution_m: f32) -> (usize, usize, usize) {
    let xpos_m = 0.043; // m
    let ypos_m = 0.050; // m
    let xwidth_m = 0.014; // m

    let xpos = xpos_m / resolution_m;
    let ypos = ypos_m / resolution_m;
    let xwidth = xwidth_m / resolution_m;

    return (xpos as usize, ypos as usize, xwidth as usize);
}

fn draw(
    width: usize,
    height: usize,
    step_nr: u32,
    present: &Vec<f32>,
) -> Result<(), Box<dyn Error>> {
    let gradient = colorous::TURBO;

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let (x, y) = (x as usize, y as usize);

        let scale = 0.33; // Max 0.5 is highest if data is +-1
        let single_array_coord: usize = x + (width as usize) * y;
        let scaled_pixel: f64 = ((present[single_array_coord]) * scale) as f64;

        let Color { r, g, b } = gradient.eval_continuous(scaled_pixel + 0.5);
        *pixel = image::Rgb([r, g, b]);
    }

    let filename = format!("output/output-{:04}.png", step_nr);
    imgbuf.save(filename)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let (width, height, wavepropagationfactor, dt_s, resolution_m) = physics();
    let (xpos, ypos, xwidth) = sound_source(resolution_m);

    // Create buffers:
    let mut past: Vec<f32> = vec![0.0; (width * height) as usize]; // Time minus 1
    let mut present: Vec<f32> = vec![0.0; (width * height) as usize]; // Current time
    let mut future: Vec<f32> = vec![0.0; (width * height) as usize]; // Time plus 1

    println!("Size: {}x{}", width, height);

    let start = Instant::now();
    let steps = 500;
    for i in 0..steps {
        let time = (i as f32) * dt_s;
        //println!("Time: {}", time);

        // Create some sound:
        let source1_offset: usize = ypos * width as usize;
        for i in xpos..xpos + xwidth {
            present[i + source1_offset] = 1.0 * (2.0 * PI * 40000.0 * time as f32).sin();
        }

        // Normal wave update step
        step(
            width,
            height,
            wavepropagationfactor,
            &past,
            &present,
            &mut future,
        );

        std::mem::swap(&mut past, &mut present); // Move present to past
        std::mem::swap(&mut present, &mut future); // Move future to present

        draw(width, height, i, &present)?;
    }
    drop(future); // Result is in present, not future, so ensure this is not used later

    let elapsed = start.elapsed();
    println!("Total elapsed time: {} ms", elapsed.as_millis());

    Ok(())
}
