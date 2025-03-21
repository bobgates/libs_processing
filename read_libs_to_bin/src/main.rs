mod import_sciaps_to_bin;
mod search_by_lambda;
mod bin_to_memory;
mod utils;

#[warn(unused_imports)]
// use import_sciaps_to_bin::{get_file_wavelength_intensity, import_sciaps_to_bin};

// Some defines for testing and to illustrate use
#[allow(dead_code)]
const SCAN1: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/338-376";
#[allow(dead_code)]
const SCAN2: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/377-419";
#[allow(dead_code)]
const SCAN3: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/420-480";
#[allow(dead_code)]
const SCAN4: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/481-540";
#[allow(dead_code)]
const SCAN5: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/541-588";



const DATASET1 : &str= "/Users/drv201/Code/libs_processing/read_libs_to_bin/data/scan1";
const DATASET2 : &str= "/Users/drv201/Code/libs_processing/read_libs_to_bin/data/scan2";
const DATASET3 : &str= "/Users/drv201/Code/libs_processing/read_libs_to_bin/data/scan3";
const DATASET4 : &str= "/Users/drv201/Code/libs_processing/read_libs_to_bin/data/scan4";
const DATASET5 : &str= "/Users/drv201/Code/libs_processing/read_libs_to_bin/data/scan5";

const DATASET_TEST : &str= "/Users/drv201/Code/libs_processing/read_libs_to_bin/data/test_data";




pub fn main() {

    // let data = import_sciaps_to_bin(SCAN1, DATASET1, false).unwrap();




    let results = search_by_lambda::search_by_lambda(DATASET4, [312.29, 242.80, 267.59].to_vec());

    // let data = get_counts()
}


