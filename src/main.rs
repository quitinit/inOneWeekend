// ppm writer

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    // image
    const IMAGEWIDTH: u32 = 256;
    const IMAGEHEIGHT: u32 = 256;
    // render
    let mut output = String::new();
    let first_line = format!("P3\n{IMAGEWIDTH} {IMAGEHEIGHT}\n255\n");
    output.push_str(&first_line);
    for j in 0..IMAGEHEIGHT {
        println!("\rScanlines remaining: {}", IMAGEHEIGHT - j);
        for i in 0..IMAGEWIDTH {
            let r = i as f64 / (IMAGEWIDTH - 1) as f64;
            let g = j as f64 / (IMAGEHEIGHT - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;
            let line = format!("{ir} {ig} {ib}\n");
            output.push_str(&line);
        }
    }
    let filepath = Path::new("./example.ppm");
    let mut file = File::create(filepath)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}
