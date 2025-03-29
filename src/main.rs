use std::fs::File;
use std::path::Path;
use esri_ascii_grid::ascii_file::EsriASCIIReader;
use image::{GrayImage, Luma, RgbImage, Rgb};
// use show_image::{ImageView, ImageInfo, create_window};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let image = ascii_to_image("test_data/LITTO3D_FRA_0926_6224_MNT_20150529_LAMB93_RGF93_IGN69.asc").expect("Failed to create image");
    // display_image(&image)?;

    // I wasn't able to display the image because I had some issue with X11 setup
    // I'll use save image instsead
    image.save("output.png").expect("Failed to save image");
    println!("Image saved as output.png");

    Ok(())
}

fn ascii_to_image<P: AsRef<Path>>(path: P) -> Result<GrayImage, String> {
    // Open the file and parse the grid
    let file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut grid: EsriASCIIReader<File, f64, f64> =
        EsriASCIIReader::from_file(file).map_err(|e| format!("Failed to read ASCII grid: {}", e))?;

    let header = grid.header;
    let nodata = header.no_data_value().unwrap_or(f64::NAN);
    let rows = header.num_rows();
    let cols = header.num_cols();

    let mut values = vec![vec![0.0; cols]; rows];
    let mut min_val = f64::MAX;
    let mut max_val = f64::MIN;

    // Read the grid values
    for cell in grid.into_iter() {
        if let Ok((row, col, value)) = cell {
            values[row][col] = value;
            if value != nodata {
                min_val = min_val.min(value);
                max_val = max_val.max(value);
                // println!("Row: {}, Col: {}, Value: {}", row, col, value);
            }
        }
    }
    // Create a new grayscale image
    let mut img = GrayImage::new(cols as u32, rows as u32);
    for row in 0..rows {
        for col in 0..cols {
            let value = values[row][col];
            let pixel = if value == nodata {
                0 // Black
            } else {
                // Normalize the value to the range [0, 255]
                ((value - min_val) / (max_val - min_val) * 255.0).round() as u8 
            };
            img.put_pixel(col as u32, (rows - 1 - row) as u32, Luma([pixel]));
        }
    }

    Ok(img)
}

fn display_image(img: &GrayImage) -> Result<(), Box<dyn std::error::Error>> {
    // Convert grayscale to RGB because GrayImage is not supported by show-image
    // Create a new RGB image
    let mut rgb_img = RgbImage::new(img.width(), img.height());
    for (x, y, gray_pixel) in img.enumerate_pixels() {
        let gray = gray_pixel[0];
        rgb_img.put_pixel(x, y, Rgb([gray, gray, gray]));
    }

    // Display using show-image
    let info = show_image::ImageInfo::rgb8(rgb_img.width(), rgb_img.height());
    let view = show_image::ImageView::new(info, rgb_img.as_raw());
    let window = show_image::create_window("Grayscale as RGB", Default::default())?;
    window.set_image("image", view)?;
    window.wait_until_destroyed()?;
    Ok(())
}