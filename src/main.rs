use std::fs::File;
use esri_ascii_grid::ascii_file::EsriASCIIReader;

fn main(){
    // Open the ESRI ASCII grid file
    let file = File::open("test_data/LITTO3D_FRA_0925_6223_MNT_20150529_LAMB93_RGF93_IGN69.asc").expect("Failed to open file");
    let mut grid: EsriASCIIReader<File, f64, f64> =
        EsriASCIIReader::from_file(file).expect("Failed to read ASCII grid");

    // Print header metadata
    let header = grid.header;
    println!("Rows: {}", header.num_rows());
    println!("Cols: {}", header.num_cols());
    println!("Cell size: {}", header.cell_size());
    println!("NoData value: {:?}", header.no_data_value());

    // Get a value at a specific row and column
    let row = 5;
    let col = 2;
    match grid.get_index(row, col) {
        Ok(value) => println!("Value at row {}, col {}: {}", row, col, value),
        Err(e) => println!("Error getting index value: {}", e),
    }

    // Get a value at a specific coordinate
    let x = 390_000.0;
    let y = 344_000.0;
    match grid.get(x, y) {
        Some(value) => println!("Value at coordinate ({}, {}): {}", x, y, value),
        None => println!("No value at coordinate ({}, {})", x, y),
    }

    // Interpolated value
    let interp_x = header.min_x() + header.cell_size() / 2.0;
    let interp_y = header.min_y() + header.cell_size() / 2.0;
    match grid.get_interpolate(interp_x, interp_y) {
        Some(value) => println!("Interpolated value at ({}, {}): {}", interp_x, interp_y, value),
        None => println!("Could not interpolate at ({}, {})", interp_x, interp_y),
    }

    // Count how many values are not NoData
    let mut valid_count = 0;
    let nodata = header.no_data_value().unwrap_or(f64::NAN);
    for cell in grid.into_iter() {
        if let Ok((_row, _col, value)) = cell {
            if value != nodata {
                valid_count += 1;
            }
        }
    }

    println!("Number of valid (non-NoData) cells: {}", valid_count);

    println!("min_x: {}", header.min_x());
    println!("max_x: {}", header.max_x());
    println!("min_y: {}", header.min_y());
    println!("max_y: {}", header.max_y());

}

