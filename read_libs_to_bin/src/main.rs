// use byteorder::LittleEndian;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Write, Read};
// use std::io::Cursor;
use std::io::BufRead;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::exit;

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
#[allow(dead_code)]









// Create a convenience to read lines from file, one at a time
// by returning a BufReader, which can be iterated over.
#[allow(dead_code)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

// Gets all the data from one file into a Vec of (wavelength, intensity)
fn get_file_wavelength_intensity(filename: &str)->Vec<(f32, f32)> 
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

// Gets all the data from one file into a Vec of (wavelength, intensity)
fn get_file_intensity(filename: &str)->Vec<f32> 
{
    let lines = read_lines(filename).unwrap();

    let mut data: Vec<f32> = Vec::new();
    let mut first = true;
 
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
pub fn get_shot_prefix(path: &str)-> Vec<String>{
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
/// every file in the folder. It returns a list of all the LIBS data
/// files that it finds.
fn get_directory_of_data_filenames(fileroot: &str)->Vec<String>{

    let shot_prefix = get_shot_prefix(fileroot);
    // let shots = (17..=32).rev();     // Example of downwards counting of shots.
    let shot_no = 1..=256;

    let mut filenames: Vec<String> = Vec::new();

    for shot in shot_prefix {
        // println!("Starting new location. Prefix is {}", shot);
        for suffix in shot_no.clone(){
            let filename = format!("{}/{}{}.csv", fileroot, &shot, suffix);
            let path = Path::new(&filename); // Does file actually exist?
            if !path.exists() {
                continue;           // If not, don't push the name into the result
            }
            filenames.push(filename);
            // println!("{}/{}{}.csv", fileroot, &shot, suffix);
        }
    }
    filenames
}






// Process all the files in a directory into a single
// binary file for faster access.
// 
// Plan:
// - Get first file, with wavelength and intensity
// - then get only intensity from all later files


fn write_to_bin() -> std::io::Result<()> {

    let mut file_path = PathBuf::new();
    
    // let path: PathBuf = ["/", "Users", "drv201", "libs_data", "CSIR_Results", "Interpolated (All Raster Shots)", "bin", ]

    let path = PathBuf::from("/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/bin/");
    let file_path = path.join("wavelengths.bin");

    // let file_path = dir.path().join("test.bin");
    let /*mut*/ original_data = vec![1.23, 4.56, 7.89];
    write_bin(original_data.iter(), &file_path)?;
    //original_data[1] = 666.66;
    let reloaded_data: Vec<f32> = read_bin(&file_path).collect();
    assert_eq!(original_data, reloaded_data);
    Ok(())
}

fn write_wavelengths(lambdas : Vec<f32>) -> std::io::Result<()> {

    let mut file_path = PathBuf::new();
    
    // let path: PathBuf = ["/", "Users", "drv201", "libs_data", "CSIR_Results", "Interpolated (All Raster Shots)", "bin", ]

    let path = PathBuf::from("/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/bin/");
    let file_path = path.join("wavelengths.bin");

    // let file_path = dir.path().join("test.bin");
    // let /*mut*/ original_data = vec![1.23, 4.56, 7.89];
    write_bin(lambdas.iter(), &file_path)?;
//    let reloaded_data: Result<Vec<f32>, _> = read_bin(&file_path)?.collect();
//    assert_eq!(original_data, reloaded_data.unwrap());
    Ok(())
}

/// Code from the internet that allows to write f32s to binary files
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=48fb576c4fa0fce24d584eebf26529d1
pub fn write_bin<'a>(data: impl Iterator<Item = &'a f32>, path: &std::path::PathBuf) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for datum in data {
        let bytes = datum.to_be_bytes();
        file.write(&bytes)?;
    }
    Ok(())
}


type IORes<T> = std::io::Result<T>;

pub fn read_bin<'a>(path: &std::path::PathBuf) -> Box<dyn Iterator<Item = f32>>{  
    
    let mut file = File::open(path).unwrap();
    let mut buffer = [0; 4];

    Box::new(std::iter::from_fn(move || {
        match file.read_exact(&mut buffer){
            Ok(()) => Some(f32::from_be_bytes(buffer)),
            Err(error) => None,
        } 
    }))
}


pub fn main() -> std::io::Result<()>{
    let path = SCAN5;
    let filenames = get_directory_of_data_filenames(path);
    let n = filenames.len();
    println!("{n} files found");
    println!("First filename is {}", filenames[0]);

    let lambda_i = get_file_wavelength_intensity(&filenames[0]); 
    let mut lambdas: Vec<f32> = Vec::new();
    for i in lambda_i {
        lambdas.push(i.0);
    }


    let path = PathBuf::from("/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/bin/");
    let file_path = path.join("wavelengths.bin");

    println!("Filepath: {}", file_path.display());


    // let /*mut*/ original_data = vec![1.23, 4.56, 7.89];
    write_bin(lambdas.iter(), &file_path)?;

    let reloaded_data: Vec<f32> = read_bin(&file_path).collect::<Vec<_>>();
    for i in 0..lambdas.len(){
        if lambdas[i]!= reloaded_data[i] {
            println!("Error in entry {} of reloaded data", i);
            exit(-1);
        }
    }
    println!("Reloaded data was without error");
   Ok(())
}

// fn main_() {

// //     bin_iter();
// //     return;
// // }

//     //println!("Hello, world!");
//     let path = SCAN5;


//     // let shot_prefix = get_shot_prefix(path);
//     // for i in 0..10 {
//     //     println!("Shot prefix: {}", shot_prefix[i]);
//     // }

//     let filenames = get_directory_of_data_filenames(path);
//     let n = filenames.len();
//     println!("{n} files found");
//     println!("First filename is {}", filenames[0]);

//     let lambda_i = get_file_wavelength_intensity(&filenames[0]); 
//     let mut lambdas: Vec<f32> = Vec::new();

//     for i in lambda_i {
//         lambdas.push(i.0);
//     }

//     write_wavelengths(lambdas); 

// // test code: write first 100 wavelengths, then read them back:

//     let mut f = File::create("/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/bin/first.bin");
//     // let cursor = Cursor::new(&mut f);
//     cursor.write_f32::<LittleEndian>(10).unwrap;


//     match f {
//         Err(e) => println!("Unable to open bin file for writing, error: {:?}",e),
//         Ok(t) => println!("Opened file okay"),
//     }

//     println!("Number of wavelengths is {}", lambda_i.len());
    

// }
