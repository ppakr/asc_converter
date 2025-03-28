use std::fs::File;
use esri_ascii_grid::ascii_file::EsriASCIIReader;

fn main(){
    // Open the ESRI ASCII grid file
    let file = File::open("test_data/test.asc").expect("Failed to open file");
    let mut grid: EsriASCIIReader<File, f64, f64> =
        EsriASCIIReader::from_file(file).expect("Failed to read ASCII grid");

    // Print header metadata
    let header = grid.header;
    println!("Rows: {}", header.num_rows());
    println!("Cols: {}", header.num_cols());
    println!("Cell size: {}", header.cell_size());
    println!("NoData value: {:?}", header.no_data_value());
}

