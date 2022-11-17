/*
    This is my program to analyse data from downloaded csv file and make a synthesized csv file
    in the pedometer directory
    
    2022.11.17      Sven Ponelat

    run it like: delete_files_list list.txt


*/

mod library;

use library::my_utils::*;
use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::*;
use std::io::{prelude::*, BufReader};



pub const VERSION: &str            = env!("CARGO_PKG_VERSION");



fn main() {

    let arguments: Vec<String> = env::args().collect();
    let infile:  &str = "/home/me/Downloads/activity-export-daily.csv"; 
    let outfile: &str = "/DATA/pedometer/activity-export.csv"; 

    sort_out_arguments(&arguments);








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















