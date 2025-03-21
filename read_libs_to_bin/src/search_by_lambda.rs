use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

/// This file searches binary data files looking for
/// file that have a specified lambda over a specified value

// #[warn(unused_imports)]
// use std::process::exit;
use crate::bin_to_memory::{get_counts, F32Reader, get_lambdas};

const DATASET : &str= "/Users/drv201/Code/libs_processing/read_libs_to_bin/data/scan4";
const DATAFILE : &str = "/scan4_amplitudes.bin";




/// given a list of wanted wavelengths and a pointer to the wavelengths file, 
/// this loads all the lambdas, then goes through them to find the wanted
/// lambdas and returns a vector of indexes into the lambdas file, and hence
/// also into the matching counts file.
/// 
/// Returns the number of lambdas in the original file, for use later
/// as well as the Vec of indexes for the lambdas that are wanted.

pub fn get_lambda_indexes(wanted_lambdas: &Vec<f32>, lambdas_filename: &str)-> (u64, Vec<Option<u64>>){

    println!("In get_lambda_indexes");

    let all_lambdas = get_lambdas(&lambdas_filename);
// println!("{} lambdas passed in", all_lambdas.len());
    let lambda_pairs: Vec<_> = all_lambdas.windows(2).collect(); 
// println!("wavelengths: {}+1", all_lambdas.len());

    let mut result: Vec<Option<u64>> = Vec::new();
    // let mut index = 0;
    let mut found = false;
    for (i,p) in lambda_pairs.into_iter().enumerate() {
        for w in wanted_lambdas{
            let a: f32 = p[0];
            let b: f32 = p[1];
            if *w>=a && *w<b {
                // println!("Wanted: {}, index: {i}, a = {a} b= {b}", *w);
                result.push(Some(i as u64));
                found = true;
                break;
            }
        }
    }
    if !found {
        result.push(None)
    }
    (all_lambdas.len() as u64, result)
}


pub fn get_lambda(wanted_lambdas: &Vec<f32>, lambdas_filename: &str)-> (u64, Vec<Option<u64>>){

    let filename = format!("{}{}", DATASET, DATAFILE);
    println!("Opening: {}", filename);

    let mut input = BufReader::new(
        File::open(filename)
        .expect("Failed to open file")
    );
    let mut buffer = [0u8; std::mem::size_of::<u64>()];
    input.read_exact(&mut buffer).unwrap();
    let n_spectra = u64::from_be_bytes(buffer);
    input.read_exact(&mut buffer).unwrap();
    let n_lambdas = u64::from_be_bytes(buffer);

    println!("Number of wavelengths in file: {}",n_lambdas);
    println!("Number of spectra in file: {}",n_spectra);

    
//---------------------------------------------------------------------------------
    let all_lambdas = get_lambdas(&lambdas_filename);

    let data: Vec<Option<u64>> = Vec::new();

    (n_lambdas, data)
}


/// search_by_lambda takes a list of wavelengths (and binary files of wavelength and
/// intensity on disk) and produces a list of amplitudes, across all the data, for 
/// the given wavelengths. 
///
/// It's designed for a smallish number of wavelengths, but that shouldn't make 
/// too much difference.
///
/// pub struct CountsPerLambda {
///     lambda: f32,
///     counts: Vec<f32>,
/// }
///
/// This takes a vector of f32 wavelengths in um for which we want data, and produces 
/// a vector of vectors, where each of the inner vectors corresponds to all the
/// results for a particular wavelength
pub fn search_by_lambda(fileroot: &str, wanted_lambdas : Vec<f32>) {//-> Vec<Vec<f32>>{

    println!("fetching data from {}", fileroot);
    let lambdas_filename = format!("{}/scan4_wavelengths.bin",fileroot); 
    println!("filename: {}", lambdas_filename);
    let lambda_info = get_lambda_indexes(&wanted_lambdas, &lambdas_filename);
    println!("lambda info: {:?}", lambda_info);

    let length = lambda_info.0;
    let indices = lambda_info.1;

println!("length and indices: {:?}", length);

    let data: Vec<Vec<f32>>= Vec::new();
    if indices.len()<1 {    // No good data, just send empty data
        // return data
    }

 
    println!("Separating out the following {} wavelengths, sorted by data index", data.len());
    for i in 0..indices.len() {
        println!("{}: {} nm ", indices[i].unwrap(), wanted_lambdas[i]);
    }
    println!();
    // indices contains the vec of the indices of all the lambda values
    // that we wish to extract from the data.

    let signals_filename = format!("{}/scan4_amplitudes.bin",fileroot); 
    let data = get_counts(&signals_filename, indices.len() as u64);


    let input = File::open(signals_filename).unwrap();
    let lambdas: Vec<f32> = F32Reader::new(BufReader::new(input)).collect();

    
    println!("get_counts: first length: {}, second length: {}", data.len(), data[1].len());


    let one_lambda : Vec<f32> = Vec::new();

    let mut count = 0;
    let mut output : Vec<f32> = Vec::new();
    for d in &data {
        // println!("{}", d[1884]);
        output.push(d[1884]);
        count+=1;
    }
    println!("Total lines read: {}", count);

    
    // for d in &data{
    //     println!("Outside")
    // }


    //println!("data outside: {}, data inside: {}, {}", data.len(), data[1].len(), data[2].len());


}

// This routine can reorder the sensor data to put it in column order, always starting at the bottom:
/*  
let mut ele: u64;
for n in 0..256{
    let row = n / 16 + 1;
    let col = n % 16 + 1;
    if row % 2 == 1 {
        ele = (row - 1) * 16 + col;
    } else {
        ele = row * 16 - col + 1;
    }
    println!("{row}, {col}, {ele}");
}
*/


/*
The data is acquired bottom left to top right zigzagged


    32 ...   17
    1 ...   16


    Desired:
    16  32 48 64    80  96 112 128  144 160 176 192 208 224 240 256                                                    256
    15  31
    14  30
    13  29
    12  28
    11  27
    10  26
    9   25
    8   24
    7   23
    6   22
    5   21
    4   20
    3   19
    2   18  34 50   66  82 98 114   130  145 162 178 194 210 226 242 
    1   17  33 49   65  81 97 113   129  145 161 177 193 209 225 241 
    --------------------------------------------------------------
    

    Actual:
    256                                                          241
    225                                                          240
    224                                                          209
    193                                                          208
    192                                                          177
    161                                                          176 
    160 159                                                      145
    129 130                                                      144
    128                                                          113
    97  98                                                       112
    96  95                                                       81
    65                                                           80
    64  63                                                       49
    33  34                                                       48
    32  31  30  29  28   27  26  25  24  23  22  21  20  19  18  17
    1   2   3   4    5   6   7   8   9   10  11  12  13  14  15  16




    bottom line: 1+16r
    next 2+16r

    top line: 16+16r






*/

