use std::fs::File;
use esri_ascii_grid::ascii_file::EsriASCIIReader;

fn main() {
    let file = File::open("./test_data/test.asc").unwrap();
    let mut grid: EsriASCIIReader<File, f64, f64> = EsriASCIIReader::from_file(file).unwrap();
    // Spot check a few values
    assert_eq!(
        grid.get_index(5, 7).unwrap(),
        grid.header.no_data_value().unwrap()
    );
    assert_eq!(grid.get(390_000.0, 344_000.0).unwrap(), 141.270_004_272_460_937_5);
    assert_eq!(grid.get(390_003.0, 344_003.0).unwrap(), 135.440_002_441_406_25);
    assert_eq!(grid.get_index(996, 3).unwrap(), 135.440_002_441_406_25);
    assert_eq!(grid.get_index(999, 0).unwrap(), 141.270_004_272_460_937_5);
    // Interpolate between cells
    let val = grid.get_interpolate(grid.header.min_x() + grid.header.cell_size()/4., grid.header.min_y() + grid.header.cell_size()/4.).unwrap();
    let header = grid.header;
    let grid_size = grid.header.num_rows() * grid.header.num_cols();
    let iter = grid.into_iter();
    let mut num_elements = 0;
    for cell in iter {
        let Ok((row, col, value)) = cell else {
            panic!("your error handler")
        };
        num_elements += 1;
        if row == 996 && col == 3 {
            let (x, y) = header.index_pos(row, col).unwrap();
            assert_eq!(x, 390003.0);
            assert_eq!(y, 344003.0);
            assert_eq!(value, 135.44000244140625);
        }
        if row == header.nrows-1 && col == 0 {
            let (x, y) = header.index_pos(row, col).unwrap();
            assert_eq!(x, 390000.0);
            assert_eq!(y, 344000.0);
            assert_eq!(value, 141.2700042724609375);
        }
    }
    assert_eq!(grid_size, num_elements);
}
