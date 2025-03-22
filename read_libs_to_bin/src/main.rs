use clap::{Parser, Subcommand};
use std::fs;
use std::fs::File;
use std::io::Write;

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


#[derive(Parser)]
#[command()]
struct Cli {
    // #[command(subcommand)]
    input_folder: String,

    // in_path: std::path::PathBuf,
    out_path: std::path::PathBuf,
}

use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};



 pub fn main() {


    let data_set : &str= "/Users/drv201/Code/libs_processing/read_libs_to_bin/my_scan1";
    let wanted_lambdas = [312.29, 242.80, 267.59].to_vec();
    let output_location ="/Users/drv201/Code/libs_processing/read_libs_to_bin/data/scan1/selected_lambdas.csv";
    
    let results = search_by_lambda::search_by_lambda(data_set, [312.29, 242.80, 267.59].to_vec());

   let mut file = File::create(output_location).expect("failed to open file");

    for i in results {
        file.write_all(i.as_ref()).unwrap();
    }


    // let mut file = File::create(output_location).expect("failed to open file");
    // for line in results {
        // std::fs::write(output_location, results);
    // }
}

// struct Cli {
//     #[clap(short, long, group = "input")]
//     /// Watch the local database to see data being written to it
//     watch: bool,

//     #[clap(short, long, group = "input")]
//     /// Initialize the device from /boot/trucklog.toml. Will reboot.
//     init: bool,

//     #[clap(short, long, group = "input")]
//     /// Show the current device configuration.
//     config: bool,

//     #[clap(short, long, group = "input")]
//     /// Drop the data from local imu, beacons & gps tables. Unrecoverable!
//     drop: bool,

//     #[clap(short, long, group = "input")]
//     /// Show the network table to check whether devices know about each other
//     network: bool,
// }

// #

