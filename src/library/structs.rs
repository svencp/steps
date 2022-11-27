/*
Most of my odd structs are in here.
2022.11.17      Sven Ponelat

*/

use std::collections::{BTreeSet, BTreeMap};
use termion::{color, style};
use std::process::exit;
use std::time::{UNIX_EPOCH, Duration};
use chrono::*;
use chronoutil::*;


pub const DAY_SECS: i64          = 86400;
pub const WEEK_SECS: i64         = 604800;



pub struct Incoming {
    pub date: i64,                           
    pub steps: i64,         
    pub duration: i64,          // seconds
    pub distance: i64,          // meters
    pub calories: i64,          // kcal
    pub floors: i64,            // 3m heights               
}

impl Incoming {
    pub fn new() -> Incoming {
        Incoming {  date: 0,
                    steps: 0, 
                    duration: 0, 
                    distance: 0, 
                    calories: 0, 
                    floors: 0 
                }
    }

    



} // end of impl Colors




// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@






// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {

    use super::*;
    use std::{fs::copy};
    use substring::Substring;
    use std::fs::remove_file;

    
    // #[ignore]
    #[test]
    fn t001_new() {

        assert_eq!(4,4);
        
    }
    






















}//end of tests

















