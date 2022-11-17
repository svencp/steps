/*
Most of my odd structs are in here.
2022.08.09      Sven Ponelat

*/

use std::collections::{BTreeSet, BTreeMap};
use termion::{color, style};
use crate::library::my_utils::*;
use std::process::exit;
use std::time::{UNIX_EPOCH, Duration};
use chrono::*;
use chronoutil::*;


pub const DAY_SECS: i64          = 86400;
pub const WEEK_SECS: i64         = 604800;



// Colors
pub struct Colors {
    pub color_active_bg: color::Rgb,                            // Orange
    pub color_black_bg: color::Rgb,                             // Black
    pub color_complete_orphan: color::Rgb,                      // White
    pub color_feedback_orange: color::Rgb,                      // Orange
    pub color_recur_chain_fg: color::Rgb,                       // Light Blue
    pub color_recur_period_fg: color::Rgb,                      // Dark Blue
    pub color_tagged: color::Rgb,                               // Dark Green
    pub color_overdue: color::Rgb,                               // Dark Red
}

impl Colors {
    pub fn new() -> Colors {
        Colors { 
            color_active_bg: color::Rgb (255,255,255),
            color_black_bg: color::Rgb (255,255,255),               
            color_complete_orphan: color::Rgb (255,255,255),        
            color_feedback_orange: color::Rgb (255,255,255),
            color_recur_chain_fg: color::Rgb (255,255,255),
            color_recur_period_fg: color::Rgb (255,255,255),
            color_tagged: color::Rgb (255,255,255),
            color_overdue: color::Rgb (255,255,255),
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

















