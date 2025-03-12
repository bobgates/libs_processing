use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::LineWriter;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use plotly::{Plot, Scatter};
use plotly::common::Title;


const SCAN1: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/338-376";
const SCAN2: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/377-419";
const SCAN3: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/420-480";
const SCAN4: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/481-540";
const SCAN5: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/541-588";
const TESTSCAN: &str = "/Users/drv201/libs_data/CSIR_Results/Interpolated (All Raster Shots)/test";


// Create a convenience to read lines from file, one at a time
// by returning a BufReader, which can be iterated over.
#[allow(dead_code)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Get the master shot numbers of all the files in a directory
/// and return them as a sorted list. The master shot is composed
/// of a sequential number, the date and time of the first shot
/// in the set, and a sequential suffix for each of the shots
/// making up that array of shots.
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

/// Create an array from one number to another with the specified spacing
pub fn create_w_array(first: f32, last: f32, spacing: f32) -> Vec<f32>{

    let delta:f32 = (last - first)/spacing;
    let delta = delta.round() as usize;

    let mut lambda_labels : Vec<f32> = Vec::new();
    lambda_labels.push(first);   

    for i in 1..=delta {
        lambda_labels.push(first + spacing*(i as f32));
    }
    lambda_labels
}

/// Open a file, and return averaged data for the wavelengths to fit 
/// into the w_array defined in create_w_array. This does some funky
/// rescaling and is not used on 11 Feb 25
#[allow(dead_code)]
pub fn get_w_data(filename: &str, wanted_lambdas: Vec<f32>)->Vec<(f32, f32)>{

    let mut lines = read_lines(filename).unwrap();
    lines.next();

    let mut result: Vec<(f32, f32)> = Vec::new();

    let mut start_wanted = wanted_lambdas[0];
    let mut stop_wanted: f32 = wanted_lambdas[1];   

    for l in wanted_lambdas.into_iter().skip(1){
        stop_wanted = l;

        let mut w: f32 = 0.0;
        let mut w_previous : f32 = 0.0;
        let mut x: f32 = 0.0;
        while w < start_wanted { // Get first w after start_w
            let line = lines.next().unwrap().unwrap();
            let a: Vec<_> = line.split(",").collect();
            w_previous = w;
            w = a[0].parse::<f32>().unwrap();
            x = a[1].parse::<f32>().unwrap();
        }
        let mut fraction_after_start = (w-stop_wanted)/(start_wanted - stop_wanted);
        let mut total_fraction = fraction_after_start;
        let mut accumulated = fraction_after_start * x;
        // println!("\nstart_w: {:.2}, stop_w: {:.2}, w: {w}, fraction after start: {}, value: {}, accumulated: {}",
                // start_wanted, stop_wanted, fraction_after_start, x, accumulated);

        loop {
            let line = lines.next().unwrap().unwrap();
            let a: Vec<_> = line.split(",").collect();
            w_previous = w;
            w = a[0].parse::<f32>().unwrap();
            x = a[1].parse::<f32>().unwrap();

            if  w > stop_wanted  {
                let fraction = (stop_wanted - w_previous)/(w - w_previous);
                // println!("x: {x}, stop wanted: {stop_wanted}, w_previous: {w_previous}, w: {w}, w_previous: {w_previous}");
                // println!("fraction: {fraction}");
                total_fraction += fraction;
                accumulated += fraction * x;
                let centre = (stop_wanted+start_wanted)/2.0;
                // println!("{centre} : {accumulated}");
                // println!("accumulated: {accumulated} in {total_fraction} steps ---- average: {}", accumulated/total_fraction);
                // println!("w: {} average: {}", centre, accumulated/total_fraction);
                result.push((centre, accumulated/total_fraction));

                break;
            } else {
                accumulated += x;
                total_fraction += 1.0;
            }

        }
        start_wanted = stop_wanted;
    }

    result
}

// Gets all the lines in the data that have wavelengths equal to or greater
// than lambda_start and equal to or less than lambda_finish. For each line, 
// it puts a wavelength, signal_strength pair into its output vector.
fn get_subset(filename: &str, lambda_start: f32, lambda_finish: f32)->Vec<(f32, f32)> 
{
    let mut lines = read_lines(filename).unwrap();
    lines.next();

    let mut data: Vec<(f32, f32)> = Vec::new();
 
    loop {
        let line = lines.next().unwrap().unwrap();
        let a: Vec<_> = line.split(",").collect();
        let wavelength: f32 = a[0].parse::<f32>().unwrap();
        let intensity: f32 = a[1].parse::<f32>().unwrap();
        
        if wavelength < lambda_start {
            continue;
        }
        if wavelength > lambda_finish {
            break;
        }
        // println!("> {}, {}", wavelength, intensity);
        data.push((wavelength, intensity));
    }
    data

}

// Gets all the data from one file into a Vec of (wavelength, intensity)
fn get_file_data(filename: &str)->Vec<(f32, f32)> 
{
    let mut lines = read_lines(filename).unwrap();

    let mut data: Vec<(f32, f32)> = Vec::new();
 
    for line in lines{
        let line = line.unwrap();
        let a: Vec<_> = line.split(",").collect();
        println!("lambda:{}| intensity:{}|", a[0], a[1].trim());
        let wavelength: f32 = a[0].trim().parse::<f32>().unwrap();
        let intensity: f32 = a[1].trim().parse::<f32>().unwrap();
        data.push((wavelength, intensity));
    }
    data

}

    // println!("Data: {:?}", data);


    // plot_spectrum(&data);



    // for l in wanted_lambdas.into_iter().skip(1){
    //     stop_wanted = l;

    //     let mut w: f32 = 0.0;
    //     let mut w_previous : f32 = 0.0;
    //     let mut x: f32 = 0.0;
    //     while w < start_wanted { // Get first w after start_w
    //         let line = lines.next().unwrap().unwrap();
    //         let a: Vec<_> = line.split(",").collect();
    //         w_previous = w;
    //         w = a[0].parse::<f32>().unwrap();
    //         x = a[1].parse::<f32>().unwrap();
    //     }
    //     let mut fraction_after_start = (w-stop_wanted)/(start_wanted - stop_wanted);
    //     let mut total_fraction = fraction_after_start;
    //     let mut accumulated = fraction_after_start * x;
    //     // println!("\nstart_w: {:.2}, stop_w: {:.2}, w: {w}, fraction after start: {}, value: {}, accumulated: {}",
    //             // start_wanted, stop_wanted, fraction_after_start, x, accumulated);

    //     loop {
    //         let line = lines.next().unwrap().unwrap();
    //         let a: Vec<_> = line.split(",").collect();
    //         w_previous = w;
    //         w = a[0].parse::<f32>().unwrap();
    //         x = a[1].parse::<f32>().unwrap();

    //         if  w > stop_wanted  {
    //             let fraction = (stop_wanted - w_previous)/(w - w_previous);
    //             // println!("x: {x}, stop wanted: {stop_wanted}, w_previous: {w_previous}, w: {w}, w_previous: {w_previous}");
    //             // println!("fraction: {fraction}");
    //             total_fraction += fraction;
    //             accumulated += fraction * x;
    //             let centre = (stop_wanted+start_wanted)/2.0;
    //             // println!("{centre} : {accumulated}");
    //             // println!("accumulated: {accumulated} in {total_fraction} steps ---- average: {}", accumulated/total_fraction);
    //             // println!("w: {} average: {}", centre, accumulated/total_fraction);
    //             result.push((centre, accumulated/total_fraction));

    //             break;
    //         } else {
    //             accumulated += x;
    //             total_fraction += 1.0;
    //         }

    //     }
    //     start_wanted = stop_wanted;
    // }


// Very basic plot routine that just puts up an
// x,y plot from the 1st and 2nd entries in each line
// data that it is sent.
pub fn scale_added_plots(){
 // !todo
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


/// Takes file directory and a minimum and maximum wavelength
/// and gets all the files in that directory. It then averages
/// the data that lines between >= lambda_min and <= lambda_max
/// and writes it to file in the same directory called out_file.
fn average_a_set(file_dir: &str, lambda_min: f32, lambda_max: f32, out_file: &str) {

    let filenames = get_directory_of_data_filenames(file_dir);
    if filenames.len()==0 {
        println!("No files found with a root of: {}", file_dir);
        println!("Terminating");
        exit(-1);
    }
    println!("{} filenames identified for processing", &filenames.len());

    let f = &filenames[0];
    println!("First file: {}", f);

    // Initialise all_data with data from the first file:
    let mut data_accumulator = get_subset(&f, lambda_min, lambda_max);
    let all_data_len = data_accumulator.len();
    // plot_spectrum(&all_data);    Plots data to around 2500 max value
    let mut num_files = 1;
    println!("-----");

    let mut data_len: usize = 0;
    for f in filenames.into_iter().skip(1) {
        num_files = num_files + 1;
        print!("\rProcessing file {}", f);
        let data = get_subset(&f, lambda_min, lambda_max);
        data_len = data.len();

        if data_len != all_data_len {
            println!("Mismatch between length of contents of file: {} and length of data accumulator", &f);
            break;
        }

        for i in 0..data_len {
            if data_accumulator[i].0 != data[i].0 {
                println!("In file {}: mismatch between wavelength {} and data accumulator {}", 
                                &f, data[i].0, data_accumulator[i].0);
                break;                    
            }
            data_accumulator[i].1 += data[i].1;     
        }        
    }

    println!("\nThere are {} files", num_files);
    for i in 0..data_len {
        data_accumulator[i].1 = data_accumulator[i].1/(num_files as f32);
    }
    // for mut all_data_i in data_accumulator.clone().into_iter(){
    //     all_data_i.1 = all_data_i.1/(num_files as f32);
    // }

    let full_out_file = file_dir.to_owned()+"/"+out_file;
    let file = File::create(full_out_file).unwrap();
    let mut file = LineWriter::new(file);
    for line in &data_accumulator {
    // Change this to overwrite the previous, maybe, so it doesn't generate 12000 lines of text
        let _ = file.write_all(format!("{}, {}\n", line.0, line.1).as_bytes());
    }
    plot_spectrum(&data_accumulator);
}

fn plot_spectrum(line: &Vec<(f32, f32)>){
    // println!("line: {:?}", line);
    let mut w: Vec<f32> = Vec::new();
    let mut intensity: Vec<f32> = Vec::new();
    for i in line {
        w.push(i.0);
        intensity.push(i.1);
    }

    let line_plot = Scatter::new(w, intensity).name("Spectrum");

    let mut plot = Plot::new();
    plot.add_trace(line_plot);
    plot.set_layout(plotly::Layout::new().title(Title::from("Intensity")));
    plot.show();
}

// A better plot routine that takes a data filename, a title for the plot
// wavelength limits and neatly plots it nicely.
pub fn plot_neatly(filename: &str, title: &str, lambda_lines: Vec<f32>){//}, markers: Vec<(f32, String)>, limits: Option<(f32, f32)>){

    let data = get_file_data(filename);
    let mut w: Vec<f32> = Vec::new();
    let mut intensity: Vec<f32> = Vec::new();
    for d in data {
        w.push(d.0);
        intensity.push(d.1);
    }


    let line_plot = Scatter::new(w, intensity).name("Spectrum");

    let mut plot = Plot::new();
    plot.add_trace(line_plot);
    plot.set_layout(plotly::Layout::new().title(Title::from("Intensity")));
    plot.show();

}

// Fixed to fetch the main three gold lines. Will alter to make more
// flexible at a later date. !todo

pub fn get_index_of_lambda(lambda: f32)->usize {

    if lambda >= 267.59 && lambda <= 267.61 {         // 267.59
        return 2599;
    } else if lambda >= 242.79 && lambda <= 242.82{     // 242.80
        return 1886;
    }
    return 3970;        // 312.29
}



pub fn fetch_a_line(file_dir: &str, lambda: f32, fout_file: &str){
    let filenames = get_directory_of_data_filenames(file_dir);
    if filenames.len()==0 {
        println!("No files found with a root of: {}", file_dir);
        println!("Terminating");
        exit(-1);
    }
    println!("{} filenames identified for processing", &filenames.len());
    println!("output file: {} ", fout_file);

    let line_no = get_index_of_lambda(lambda);

    let mut data : Vec<f32> = Vec::new();

    let mut previous_wavelength: f32 = 0.0;

    for filename in filenames {
        println!("filename: {}", filename);
        let mut lines = read_lines(filename).unwrap();

        // println!("{} lines read", lines.len());



        let mut first = true;       // need to skip first line that has headers in it.
        for line in lines{
            let line = line.unwrap();
// println!("lambda: {}, line: {}", lambda, line);
            if !first {
                let a: Vec<_> = line.split(",").collect();
                // println!("lambda:{}| intensity:{}|", a[0], a[1].trim());
                let wavelength: f32 = a[0].trim().parse::<f32>().unwrap();
                let intensity: f32 = a[1].trim().parse::<f32>().unwrap();
                if lambda > previous_wavelength && lambda < wavelength {
                    data.push(intensity);
                    break;
                }
                let previous_wavelength = wavelength;
            }
            first = false;
        }
    }

    let file = File::create(fout_file).unwrap();
    let mut file = LineWriter::new(file);
    for line in &data {
    // Change this to overwrite the previous, maybe, so it doesn't generate 12000 lines of text
        let _ = file.write_all(format!("{}\n", line).as_bytes());
    }
    plot_wavelength(lambda, data);

}

pub fn plot_wavelength(wavelength: f32, intensity_in : Vec<f32>){
    let mut intensity: Vec<f32> = Vec::new();
    let mut w: Vec<f32> = Vec::new();

    let mut index: f32=0.0;
    for i in &intensity_in {
        w.push(index);
        index+=1.0;
        intensity.push(*i);         // bad name !todo
    }

    // println!("w:{:?}, intensity: {:?}", w, intensity);
    let line_plot = Scatter::new(w, intensity).name("Along sample");

    let mut plot = Plot::new();
    plot.add_trace(line_plot);
    plot.set_layout(plotly::Layout::new().title(Title::from("Along sample")));
    plot.show();
}

pub struct Line{
    wavelength: f32,
    name: String,
}

enum ChooseToRun {
    ProcessAverageData,
    ProcessOneWavelength,
    NeatPlot,
}

pub fn main(){
    // Gold peaks are at 242.79440, 267.59366, 312.27831 nm
    // WORKS: 
    // average_a_set(SCAN5, 240.0, 320.0, "average_240_320");


    let choice : ChooseToRun = ChooseToRun::ProcessAverageData; //ChooseToRun::ProcessOneWavelength;

    let mut lambda_lines : Vec<f32> = Vec::new();
    lambda_lines.push(267.59366);
    lambda_lines.push(242.79440);
    lambda_lines.push(312.27831);


    let filename = format!("{}/{}", SCAN5, "average_240_320");
    let title = "LIBS Intensity - Sample 1, Gold";
    // let title = "LIBS Intensity - Sample 2, Gold";
    // let title = "LIBS Intensity - Sample 3, Gold";
    // let title = "LIBS Intensity - Sample 4, PGM";
    // let title = "LIBS Intensity - Sample 5, Gold";

    // Lines are 312.29
    // 242.80
    // and 267.59

    match choice {
        ChooseToRun::ProcessAverageData => 
            average_a_set(SCAN4, 240.0, 320.0, "average_240_320"),

        ChooseToRun::ProcessOneWavelength => {
            fetch_a_line(SCAN1, 242.80,  "242_line"); //267.59. 242.80, 312.29

        },
        ChooseToRun::NeatPlot =>  {
            plot_neatly(&filename, title, lambda_lines);// markers, limits);
        },
    };

}
