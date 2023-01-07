use std::fs::File;
use std::io::{Result, Write};

#[allow(dead_code)]
pub const EPSILON: f32 = 0.00001;

pub fn write_to_file(path: &str, content: &str) -> Result<()>{
    let mut file = File::create(path)?;
    file.write(content.as_bytes())?;
    Ok(())
}
