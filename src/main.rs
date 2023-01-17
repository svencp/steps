/*
    This is my program to analyse data from downloaded csv file and make a synthesized csv file
    in the pedometer directory
    
    2022.11.17      Sven Ponelat

    


*/

mod library;

use library::structs::*;
use local_timestamps::*;
use csv_csp::*;
use error_feedback::*;
use substring::Substring;
use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::*;
use std::io::{BufWriter, Write};
use std::collections::BTreeMap;
use chrono::*;


pub const VERSION: &str            = env!("CARGO_PKG_VERSION");



fn main() {
    let arguments: Vec<String> = env::args().collect();
    let infile:  &str = "/DATA/pedometer/daily.csv"; 
    let num_fields = 6;
    let skip = 2;
    let delimiter = ";";
    let outfile: &str = "/DATA/pedometer/months.csv"; 
    let mut baskets: BTreeMap<i64,Basket> = BTreeMap::new();

    sort_out_arguments(&arguments);

    // get the vector of strings
    let res_load = csv_get_str_vec(infile, num_fields, skip, delimiter);
    if res_load.is_err() {
        let message = res_load.err().unwrap().to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }

    let res_conversion = convert_from_strings(res_load.unwrap());
    if res_conversion.is_err() {
        let message = res_conversion.err().unwrap().to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }


    
    // main loop
    for day in res_conversion.unwrap() {

        let dc = day.clone();
        
        let basket = Basket::new(
                                dc.start_month,
                                dc.steps,
                                dc.distance,
                                dc.calories,
                                dc.duration,
                                dc.floors,
                                dc.steps,
                                dc.distance,
                                dc.calories,
                                dc.duration,
                                dc.floors,
                            );

        let key = &basket.start_month;                    

        match baskets.contains_key(&key) {

            // need to update
            true => {
                let mut modified = baskets.remove_entry(key).unwrap().1;

                modified.total_steps    += basket.steps; 
                modified.total_distance += basket.distance; 
                modified.total_calories += basket.calories; 
                modified.total_duration += basket.duration; 
                modified.total_floors   += basket.floors; 

                // comparisons
                if basket.steps > modified.steps {
                    modified.steps = basket.steps;
                } 

                if basket.distance > modified.distance {
                    modified.distance = basket.distance;
                }

                if basket.calories > modified.calories {
                    modified.calories = basket.calories;
                }

                if basket.duration > modified.duration {
                    modified.duration = basket.duration;
                }

                if basket.floors > modified.floors {
                    modified.floors = basket.floors;
                }

                baskets.insert(*key, modified);
            }

            // need to create a new entry
            false => {
                baskets.insert(basket.start_month, basket);
            }
        }
    }
    
    // output file
    let write_res = write_outfile(outfile,&baskets);
    if write_res.is_err() {
        let message = write_res.err().unwrap().to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }



}





// ***************************************** Functions **************************************************
pub fn sort_out_arguments(arguments: &Vec<String>) {

    // Check on arguments
    match arguments.len() {
        // no arguments
        1 => {
            let message = "All good! No arguments given.".to_string();
            feedback(Feedback::Info, message);
        }
        2 => {
            match arguments[1].as_str() {
                "ver" | "-v" | "-ver" | "v" => {
                    let message = format!("Version is {}",VERSION);
                    feedback(Feedback::Info, message);
                    exit(17);
                }
                _ => {
                    // Garbage if version is not required
                    let message = "Only version argument is allowed!".to_string();
                    feedback(Feedback::Error, message);
                    exit(17);
                }
            }
        }
        _ => {
            let message = "Too many arguments (parameters) given!".to_string();
            feedback(Feedback::Error, message);
            exit(17);
        }
    }
}


// // The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }


// a function to convert from string vector to a vector of structs
pub fn convert_from_strings(array: Vec<Vec<String>>) -> Result<Vec<Incoming>,String> {
    let mut ret: Vec<Incoming> = Vec::new();  
    let mut item_counter = 0;


    for day in array {
        item_counter += 1;
        let mut incoming = Incoming::new();

        // date
        let index = day.clone()[0].find("+").unwrap();
        let clean = day[0].substring(0, index).trim();
        let r_date = lts_date_time_string_to_timestamp(clean);
        if r_date.is_err() {
            let message = format!("The date component of item number {} did not convert!",item_counter).to_string();
            return Err(message);
        }
        incoming.date        = r_date.clone().unwrap();
        incoming.start_month = lts_get_first_of_month_timestamp(r_date.unwrap());

        // steps
        let r_steps = day[1].parse::<i64>();
        if r_steps.is_err() {
            let message = format!("The steps component of item number {} did not convert!",item_counter).to_string();
            return Err(message);
        }
        incoming.steps = r_steps.unwrap();

        // duration 
        let r_dur = day[2].parse::<i64>();
        if r_dur.is_err() {
            let message = format!("The duration component of item number {} did not convert!",item_counter).to_string();
            return Err(message);
        }
        incoming.duration = r_dur.unwrap();

        // distance
        let r_dist = day[3].parse::<i64>();
        if r_dist.is_err() {
            let message = format!("The distance component of item number {} did not convert!",item_counter).to_string();
            return Err(message);
        }
        incoming.distance = r_dist.unwrap();

        // calories
        let r_cal = day[4].parse::<i64>();
        if r_cal.is_err() {
            let message = format!("The calories component of item number {} did not convert!",item_counter).to_string();
            return Err(message);
        }
        incoming.calories = r_cal.unwrap();

        // floors
        let r_floors = day[5].parse::<i64>();
        if r_floors.is_err() {
            let message = format!("The floors component of item number {} did not convert!",item_counter).to_string();
            return Err(message);
        }
        incoming.floors = r_floors.unwrap();


        // add to vector
        ret.push(incoming);
    }

    Ok(ret)
}

#[allow(deprecated)]
// write output to csv file
pub fn write_outfile(csv_file: &str, months: &BTreeMap<i64,Basket>) -> Result<(),String> {
    let ch: char = ',';
    let path = Path::new(csv_file);
    let f = match OpenOptions::new()
                                    .read(false)
                                    .write(true)
                                    .create(true)
                                    .truncate(true)
                                    .open(path)  {
        
        Err(_) => { return Err("Problem opening export to csv file".to_string()); }
        Ok(file)   => { file }
    };

    let mut file = BufWriter::new(f);

    for key in months {
        let mut line: String = String::new();
        let basket = key.1;

        let unix_timestamp = key.clone().0.to_owned();
        let timestamp = NaiveDateTime::from_timestamp(unix_timestamp, 0);
        let date_string = timestamp.format("%b %Y").to_string();

        line.push_str(date_string.as_str());
        line.push(ch);
        line.push_str(basket.total_steps.to_string().as_str());
        line.push(ch);

        let mut km = format!("{:.1}", basket.total_distance  as f64 / 1000.0);
        line.push_str(km.as_str());
        line.push(ch);
        line.push_str(basket.total_calories.to_string().as_str());
        line.push(ch);

        let mut hours = format!("{:.1}", basket.total_duration  as f64 / 3600.0);
        line.push_str(hours.as_str());
        line.push(ch);
        line.push_str(basket.total_floors.to_string().as_str());
        line.push(ch);
        line.push(ch);
        line.push(ch);

        line.push_str(basket.steps.to_string().as_str());
        line.push(ch);

        km = format!("{:.1}", basket.distance  as f64 / 1000.0);
        line.push_str(km.as_str());
        line.push(ch);
        line.push_str(basket.calories.to_string().as_str());
        line.push(ch);

        hours = format!("{:.1}", basket.duration  as f64 / 3600.0);
        line.push_str(hours.as_str());
        line.push(ch);
        line.push_str(basket.floors.to_string().as_str());
        line.push('\n');

        match file.write_all(line.as_bytes()) {
            Err(_) => { return Err("Problem writing data csv file".to_string()); } 
            _      => {  } 
        }
    }


    Ok(())

}










// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {

    use super::*;
    use std::{fs::copy};
    // use substring::Substring;
    use std::fs::remove_file;

    
    // #[ignore]
    #[test]
    fn t001_conv() {
        let source = "./test/documents/small.csv";
        let destination = "./test/work.csv";
        copy(source,destination).expect("Failed to copy");
        let res_veccy = csv_get_str_vec(destination,6, 2,";").unwrap();
        let res_conversion = convert_from_strings(res_veccy).unwrap();
        remove_file(destination).expect("Cleanup test failed");

        let two = &res_conversion[2];
        assert_eq!(two.date,1537401600);
        assert_eq!(two.steps,6958);
        assert_eq!(two.duration,4356);
        assert_eq!(two.distance,4202);
        assert_eq!(two.calories,377);
        assert_eq!(two.floors,12);
    }
    
    // #[ignore]
    #[test]
    fn t002_fail() {
        // date
        let mut source = "./test/documents/date.csv";
        let mut destination = "./test/work.csv";
        copy(source,destination).expect("Failed to copy");
        let mut res_veccy = csv_get_str_vec(destination,6, 2,";").unwrap();
        let mut res_conversion = convert_from_strings(res_veccy);
        remove_file(destination).expect("Cleanup test failed");
        let mut error = res_conversion.err().unwrap();
        assert!(error.starts_with("The date component"));

        // steps
        source = "./test/documents/steps.csv";
        destination = "./test/work.csv";
        copy(source,destination).expect("Failed to copy");
        res_veccy = csv_get_str_vec(destination,6, 2,";").unwrap();
        res_conversion = convert_from_strings(res_veccy);
        remove_file(destination).expect("Cleanup test failed");
        error = res_conversion.err().unwrap();
        assert!(error.starts_with("The steps component"));

        // duration
        source = "./test/documents/duration.csv";
        destination = "./test/work.csv";
        copy(source,destination).expect("Failed to copy");
        res_veccy = csv_get_str_vec(destination,6, 2,";").unwrap();
        res_conversion = convert_from_strings(res_veccy);
        remove_file(destination).expect("Cleanup test failed");
        error = res_conversion.err().unwrap();
        assert!(error.starts_with("The duration component"));

        // distance
        source = "./test/documents/distance.csv";
        destination = "./test/work.csv";
        copy(source,destination).expect("Failed to copy");
        res_veccy = csv_get_str_vec(destination,6, 2,";").unwrap();
        res_conversion = convert_from_strings(res_veccy);
        remove_file(destination).expect("Cleanup test failed");
        error = res_conversion.err().unwrap();
        assert!(error.starts_with("The distance component"));

        // calories
        source = "./test/documents/calories.csv";
        destination = "./test/work.csv";
        copy(source,destination).expect("Failed to copy");
        res_veccy = csv_get_str_vec(destination,6, 2,";").unwrap();
        res_conversion = convert_from_strings(res_veccy);
        remove_file(destination).expect("Cleanup test failed");
        error = res_conversion.err().unwrap();
        assert!(error.starts_with("The calories component"));

        // floors
        source = "./test/documents/floors.csv";
        destination = "./test/work.csv";
        copy(source,destination).expect("Failed to copy");
        res_veccy = csv_get_str_vec(destination,6, 2,";").unwrap();
        res_conversion = convert_from_strings(res_veccy);
        remove_file(destination).expect("Cleanup test failed");
        error = res_conversion.err().unwrap();
        assert!(error.starts_with("The floors component"));

    }




















}//end of tests