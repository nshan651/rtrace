use crate::vec3::Color;
use std::io::{self, Write};

pub fn write_color(out: &mut dyn Write, pixel_color: Color) -> io::Result<()> {
    // Write the translated [0,255] value of each color component.
    writeln!(out, "{} {} {}", (255.999 * pixel_color.x()) as u32, (255.999 * pixel_color.y()) as u32, (255.999 * pixel_color.z()) as u32)?;
    Ok(())
}
