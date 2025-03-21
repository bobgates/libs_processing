/// This file contains routines that will import two LIBS bin files into memory
/// and allow them to be used for computation

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io;
use std::io::Read;
use std::io::Write;

pub fn import_libs_bins(){}

// This neat approach to reading in binary files is from the last suggestion on:
// From https://stackoverflow.com/questions/70466567/read-binary-file-in-units-of-f64-in-rust


pub struct F32Reader<R: io::BufRead> {
    inner: R,
}

impl <R: io::BufRead> F32Reader<R>{
    pub fn new(inner: R) -> Self {
        Self {
            inner
        }
    }
}

impl <R: io::BufRead> Iterator for F32Reader<R>{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item>{
        let mut buff: [u8; 4] = [0;4];
        self.inner.read_exact(&mut buff).ok()?;
        Some(f32::from_be_bytes(buff))
    }
}


pub fn get_lambdas(filename: &str)->Vec<f32>{

    println!("Trying to get lambdas from file: {}", filename);
    let input = File::open(filename).unwrap();
    println!("okay with input");

    let lambdas: Vec<f32> = F32Reader::new(io::BufReader::new(input)).collect();
    
    // println!("lambdas: {:?}", lambdas);


    lambdas
}


/// GIVEN
pub fn get_counts(filename: &str, n_lambdas: u64)->Vec<Vec<f32>>{

    println!("in get_counts, filename is:{filename}");

    let metadata = fs::metadata(filename).unwrap();
    let l = metadata.len();

    let n_records = l/(n_lambdas*4) as u64;

// START HERE
// **********
    println!("File {} has a total of {} f32 records. Reading...", filename, n_records);

    // let filename = format!("{}{}", DATASET, DATAFILE);
    println!("Opening: {}", filename);

    let mut input = BufReader::new(
        File::open(filename)
        .expect("Failed to open file")
    );


    let mut input = File::open(filename).unwrap();
println!("File opens okay");

let mut buffer = [0u8; std::mem::size_of::<u64>()];
input.read_exact(&mut buffer).unwrap();
let n_spectra = u64::from_be_bytes(buffer);
input.read_exact(&mut buffer).unwrap();
let n_lambdas = u64::from_be_bytes(buffer);

println!("     {n_spectra}, {n_lambdas}");



// I have an unknown number of positions, where each position
// has n_lambdas of data points. So I need to read in a line of
// data, then see if there's more, etc.

    let mut data: Vec<Vec<f32>> = Vec::new();
    let mut vector: Vec<f32> = Vec::new();

    let mut v_count=0;
    let mut t_count = 0;
    
    for f in F32Reader::new(BufReader::new(input)){
        if v_count == 0 {
            vector.truncate(0); 
        }
        vector.push(f);
        v_count = v_count + 1;

        if v_count == n_lambdas {
            data.push(vector.clone());
            v_count = 0;
            t_count += 1;
            // print!("{:.1}%\r", (t_count as f32)/(n_records as f32)*100.0); 
            let _ = io::stdout().flush();
        }
    }

    println!("Data read in consisted of {} lines and {} entries per line", data.len(), n_lambdas);
    data
}

