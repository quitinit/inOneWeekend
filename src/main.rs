// ppm writer

use inOneWeekend::{write_color, Color};

use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;
fn main() -> Result<()> {
    // image
    const IMAGEWIDTH: u32 = 256;
    const IMAGEHEIGHT: u32 = 256;
    // render
    let first_line = format!("P3\n{IMAGEWIDTH} {IMAGEHEIGHT}\n255\n");
    let filepath = Path::new("./example2.ppm");
    let mut file = File::create(filepath)?;
    file.write_all(first_line.as_bytes())?;
    for j in 0..IMAGEHEIGHT {
        println!("\rScanlines remaining: {}", IMAGEHEIGHT - j);
        for i in 0..IMAGEWIDTH {
            let r = i as f64 / (IMAGEWIDTH - 1) as f64;
            let g = j as f64 / (IMAGEHEIGHT - 1) as f64;
            let b = 0.0;
            let color = Color::new(r, g, b);
            write_color(&mut file, &color)?;
        }
    }
    Ok(())
}
