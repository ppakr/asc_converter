use std::fs::File;
use std::path::Path;
use esri_ascii_grid::ascii_file::EsriASCIIReader;
use image::GrayImage;
use image::Luma;
use show_image::{ImageView, ImageInfo, create_window};

fn main(){
    // // Open the ESRI ASCII grid file
    // let file = File::open("test_data/test.asc").expect("Failed to open file");
    // // let file = File::open("test_data/LITTO3D_FRA_0925_6224_MNT_20150529_LAMB93_RGF93_IGN69.asc").expect("Failed to open file");
    // let mut grid: EsriASCIIReader<File, f64, f64> =
    //     EsriASCIIReader::from_file(file).expect("Failed to read ASCII grid");

    // // Print header metadata
    // let header = grid.header;
    // println!("Rows: {}", header.num_rows());
    // println!("Cols: {}", header.num_cols());
    // println!("Cell size: {}", header.cell_size());
    // println!("NoData value: {:?}", header.no_data_value());

    // let nodata = header.no_data_value().unwrap_or(f64::NAN);

    // let rows = header.num_rows();
    // let cols = header.num_cols();

    // // create 2d vector to store the values
    // let mut values = vec![vec![0.0; cols]; rows];
    // let mut min_val = f64::MAX;
    // let mut max_val = f64::MIN;

    // // fill values
    // for cell in grid.into_iter(){
    //     if let Ok((row, col, value)) = cell{
    //         values[row][col] = value;
    //         if value != nodata{
    //             min_val = min_val.min(value);
    //             max_val = max_val.max(value);
    //             // println!("row: {}, col: {}, value: {}", row, col, value);
    //         }
    //     }
    // }

    // // normalize values
    // let mut img = GrayImage::new(cols as u32, rows as u32);
    // for row in 0..rows {
    //     for col in 0..cols {
    //         let value = values[row][col];
    //         let pixel = if value == nodata { 0 } else {
    //             let norm = ((value - min_val) / (max_val - min_val) * 255.0).round() as u8;
    //             norm
    //         };
    //         // Flip vertically to match image coordinates
    //         img.put_pixel(col as u32, (rows - 1 - row) as u32, Luma([pixel]));
    //     }
    // }

    // // save image
    // img.save("output.png").expect("Failed to save image");
    // println!("Image saved as output.png");
    let image = ascii_to_image("test_data/test.asc").expect("Failed to create image");
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

    for cell in grid.into_iter() {
        if let Ok((row, col, value)) = cell {
            values[row][col] = value;
            if value != nodata {
                min_val = min_val.min(value);
                max_val = max_val.max(value);
            }
        }
    }

    let mut img = GrayImage::new(cols as u32, rows as u32);
    for row in 0..rows {
        for col in 0..cols {
            let value = values[row][col];
            let pixel = if value == nodata {
                0
            } else {
                ((value - min_val) / (max_val - min_val) * 255.0).round() as u8
            };
            img.put_pixel(col as u32, (rows - 1 - row) as u32, Luma([pixel]));
        }
    }

    Ok(img)
}