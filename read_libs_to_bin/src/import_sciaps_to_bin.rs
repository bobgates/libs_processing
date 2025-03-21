/// Process a set of SciAps raw data files into a single 
/// binary data file. Set up here with filenames
/// of some existing directories of files for testing 
/// purposes. 

use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;
use std::path::{Path, PathBuf};


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


// Create a convenience to read lines from file, one at a time
// by returning a BufReader, which can be iterated over.
#[allow(dead_code)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

// Gets all the data from one file into a Vec of (wavelength, intensity)
pub fn get_file_wavelength_intensity(filename: &str)->Vec<(f32, f32)> 
{
    let lines = read_lines(filename).unwrap();

    let mut data: Vec<(f32, f32)> = Vec::new();
 
    for line in lines.skip(1){
        let line = line.unwrap();
        let a: Vec<_> = line.split(",").collect();
        // println!("lambda:{}| intensity:{}|", a[0], a[1].trim());
        let wavelength: f32 = a[0].trim().parse::<f32>().unwrap();
        let intensity: f32 = a[1].trim().parse::<f32>().unwrap();
        data.push((wavelength, intensity));
    }
    data
}

// Gets all the data from one file into a Vec of (intensity)
fn get_file_intensity(filename: &str)->Vec<f32> 
{
    println!("In get_file_intensity");


    let lines = read_lines(filename).unwrap();

    let mut data: Vec<f32> = Vec::new();
    let _first = true;
 
    for line in lines.skip(1){
        let line = line.unwrap();
        let a: Vec<_> = line.split(",").collect();
        //println!("lambda:{}| intensity:{}|", a[0], a[1].trim());
        // let wavelength: f32 = a[0].trim().parse::<f32>().unwrap();
        let intensity: f32 = a[1].trim().parse::<f32>().unwrap();
        data.push(intensity);
    }
    data
}

/// Get the master shot numbers of all the files in a directory
/// and return them as a sorted list. The master shot is composed
/// of a sequential number, the date and time of the first shot
/// in the set, and a sequential suffix for each of the shots
/// making up that array of shots. For example:
/// shot file name: 000340_2024_10_18_145618_214
/// 340 is the shot set number, taken on 2024_10_18_145618
/// It contains (as it happens) 256 shots, and this is number 214
///
/// This routine returns all the shot prefixes, which can be
/// used to construct a vector of all the data file names.
pub fn get_sorted_shot_prefix(path: &str)-> Vec<String>{
    let mut shot_prefix: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(path){
        for entry in entries {
            if let Ok(entry) = entry {
                // println!("{}", entry.file_name().to_str().unwrap());
                if let Some(f) = entry.file_name().to_str() {
                    let f = f.to_string();
                    let f = f.split("_").collect::<Vec<&str>>();
                    // println!("{:?} - {}", f, f.len());
                    if f.len()>4 {
                        let f = format!("{}_{}_{}_{}_{}_", f[0], f[1], f[2], f[3], f[4]);
                        if f.chars().nth(0)==Some('0'){
                            if !shot_prefix.contains(&f){
                                shot_prefix.push(f);
                            }
                        }
                    }
                }
            }
        }
    }
    shot_prefix.sort();
    shot_prefix
}


/// This function takes a directory, and works out how to access
/// every file in the folder. 
/// 
/// First, it calls get_sorted_shot_prefix which returns the names 
/// of all the LIBS files in the folder, which are sorted, essentially
/// by name.
/// 
/// Then it returns a list of the actual filenames of the .csv files
/// that are in the folder including the
/// sub-shot number, 1-256.
/// 
/// This is not done directly, because the files don't sort alphabetically
/// _14 comes before _4 alphabetically. 

fn get_directory_of_data_filenames(fileroot: &str)->Vec<String>{

    let shot_prefix = get_sorted_shot_prefix(fileroot);
    // let shots = (17..=32).rev();     // Example of downwards counting of shots.
    let shot_no = 1..=256;

    let mut filenames: Vec<String> = Vec::new();

    for shot in shot_prefix {
        // println!("Starting new location. Prefix is {}", shot);
        for suffix in shot_no.clone(){
            let filename = format!("{}/{}{}.csv", fileroot, &shot, suffix);
            let path = Path::new(&filename); // Does file actually exist?
            if !path.exists() {
                continue;                           // If not, don't push the name into the result
            }
            filenames.push(filename);
        }
    }
    filenames
}

/// Code from the internet that writes an iterator of f32s to a binary file
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=48fb576c4fa0fce24d584eebf26529d1
pub fn write_bin<'a>(data: impl Iterator<Item = &'a f32>, mut file: &File) -> std::io::Result<&File> {
    
    for datum in data {
        let bytes = datum.to_be_bytes();
        file.write(&bytes)?;
    }
    Ok(file)
}

// This should be the first write. It puts the sizes of the vectors into the file as the first entry
pub fn write_sizes<'a>(n_spectra: u64, n_wavelengths: u64, mut file: &File) -> std::io::Result<&File> {
    
        let bytes = n_spectra.to_be_bytes();

        file.write(&bytes)?;

    Ok(file)
}


// Adds further data to an existing binary file. Use with write_bin
pub fn append_bin<'a>(data: impl Iterator<Item = &'a f32>, mut file: &File)-> std::io::Result<()> {//path: &std::path::PathBuf) -> std::io::Result<()> {
    for datum in data {
        let bytes = datum.to_be_bytes();
        file.write(&bytes)?;
    }
    Ok(())
}

/// Give this a path in and out and it'll create two files: wavelengths.bin and amplitudes.bin in 
/// that path.
pub fn import_sciaps_to_bin(input_path : &str, output_path: &str, verbose: bool)-> std::io::Result<()>{

    // First get the filenames to use for input
    let input_filenames = get_directory_of_data_filenames(input_path);
    let n_spectra = input_filenames.len() as u64;
    if verbose {
        println!("{n_spectra} files found");
        println!("First filename is {}", input_filenames[0]);
        println!("Getting wavelengths\n");
    }

    // Then get wavelengths and store them to file. This assumes that all wavelength records
    // will be the same within a 256 shot, supported by the padding in the SciAps files at
    // front and back. This assumption is checked later - data files need to have
    // the same number of entries.
    let lambda_i = get_file_wavelength_intensity(&input_filenames[0]); 
    let mut lambdas: Vec<f32> = Vec::new();
    for i in lambda_i {
        lambdas.push(i.0);
    }
    let n_wavelengths = lambdas.len() as u64;


    // Write the wavelengths out into a single file filled with 4 byte f32's of wavelength
    let output_path = PathBuf::from(output_path);
    let file_path = output_path.join("wavelengths.bin");
    if verbose {
        println!("Filepath: {}", file_path.display());
    }
    let file  =  File::create(file_path).unwrap();

    write_bin(lambdas.iter(), &file)?;
    if verbose {
        println!("Wavelengths done");
    }

    // ********************************************************************************************************
    // Now get amplitude data and start adding it to the amplitudes.bin file
    if verbose {
        println!("Getting amplitude data");
    }

    let output_path = PathBuf::from(output_path);
    let file_path = output_path.join("amplitudes.bin");
    let file = File::create(file_path)?;
    write_sizes(n_spectra, n_wavelengths, &file)?;      // First item in file, rest are appended.
    let mut first = true;
    let mut filesize: u64 = 0;
    for (i, filename) in input_filenames.iter().enumerate(){
        
        // Read a file into a vector of rx_i values and check for consistent
        // sizes file to file
        let rx_i = get_file_intensity(&input_filenames[i]);
        if first { 
            filesize = rx_i.len() as u64;
        }
        let l = rx_i.len() as u64;
        if filesize != l {
            println!("filesize is wrong for the next file");
            let errstr = format!("Error in filesize of {filename}");
            return Err(std::io::Error::new(io::ErrorKind::Other, errstr));
        }

        // Show progress if required (or even if not)
        if verbose {
            println!("{i}/{n_spectra}, {filename}");
        } else {
            print!(".");
            let _ = io::stdout().flush();
        }

        // Add the single file of intensity to the output file. 
        let mut rx_strength: Vec<f32> = Vec::new();
        // I don't know why I push f32s into a vec of 
        // f32. I'm not changing it now - it works.
        for i in rx_i{
            rx_strength.push(i);
        }
        if first {
            write_bin(rx_strength.iter(), &file)?;
            first = false;
        } else 
        
        {
            let _ = append_bin(rx_strength.iter(), &file);
        }
    }


   Ok(())
}
