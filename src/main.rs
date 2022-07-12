use std::{env};
use std::process::exit;
use std::ptr::copy_nonoverlapping;
use image::{DynamicImage, RgbaImage};

// This macro does exactly what you think it does.
macro_rules! die {
    ( $msg:expr $( ,$arg:expr )* ) => {
        eprint!("Error: ");
        eprintln!($msg $(, $arg)*);
        exit(1);
    }
}

fn clamp<T: PartialOrd> (val: T, lo: T, hi: T) -> T {
    if val < lo { lo } else if val > hi { hi } else { val }
}

struct Img { data: RgbaImage }
impl Img {
    // Create a new blank image.
    fn new(width: u32, height: u32) -> Img {
        Img { data: DynamicImage::new_rgba8(width, height).to_rgba8() }
    }

    // Load an image from a path.
    fn load(path: &str) -> Result<Self, &'static str> {
        Ok(Img {
            data: image::io::Reader::open(path).map_err(|_| "Could not load file")?
                .decode().map_err(|_| "Could not decode image")?
                .to_rgba8()
        })
    }

    // Get the height of the image.
    fn ht(&self) -> u32 { self.data.height() }

    // Save the image to a file.
    fn save(&self, path: &str) -> Result<(), &'static str> {
        self.data.save(path).map_err(|_| "Could not save image")
    }

    // Get the width of the image.
    fn wd(&self) -> u32 { self.data.width() }

    // Copy an image to this image at the given (x, y) position.
    fn write(&mut self, img: &Img, x: u32, y: u32) -> Result<(), &'static str> {
        // Make sure the position isn't out of bounds.
        if x >= self.wd() || y >= self.ht() { return Err("Invalid position"); }

        // Determine how many rows we need to copy and how many bytes each row contains.
        let row_bytes = clamp(img.wd() * 4, 0, (self.wd() - x) * 4);
        let rows = clamp(img.ht(), 0, self.ht() - y);
        if rows < 1 || row_bytes < 1 { return Ok(()); }

        // Ptrs to the image contents.
        let src_origin = img.data.as_ptr();
        let dest_origin = self.data.as_mut_ptr();

        // Copy the data to the image.
        unsafe {
            for i in 0..rows {
                let src = src_origin.offset((i * img.wd() * 4) as isize);
                let dest = dest_origin.offset((((i + y) * self.wd() + x) * 4) as isize);
                copy_nonoverlapping(src, dest, row_bytes as usize);
            }
        }
        Ok(())
    }
}

fn process_file(infile: &str, ofile: &str) -> Result<(), &'static str> {
    // Read the image.
    let img = Img::load(infile)?;

    // Duplicate it.
    let mut dup = Img::new(img.wd() * 2, img.ht());
    dup.write(&img, 0, 0)?;
    dup.write(&img, img.wd() + 50, 50)?;

    // Save the image.
    dup.save(ofile)?;
    Ok(())
}

fn main() {
    // Get the input and output file names.
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { die!("Usage: {} <input> <output>", args[0]); }
    let input_file = &args[1];
    let output_file = &args[2];

    // Process it.
    process_file(input_file, output_file).unwrap();
}