/*
        Most of my odd structs are in here.
        2022.11.17      Sven Ponelat

*/




#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Incoming {
    pub date: i64,                           
    pub steps: i64,         
    pub duration: i64,          // seconds
    pub distance: i64,          // meters
    pub calories: i64,          // kcal
    pub floors: i64,            // 3m heights          
    pub start_month: i64,       // start of the month     
}

impl Incoming {
    pub fn new() -> Incoming {
        Incoming {  date: 0,
                    steps: 0, 
                    duration: 0, 
                    distance: 0, 
                    calories: 0, 
                    floors: 0,
                    start_month: 0,
                }
    }

} // end of impl Incoming

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Basket {
    pub start_month: i64,
    pub total_steps: i64,
    pub total_distance: i64,
    pub total_calories: i64,
    pub total_duration: i64,
    pub total_floors: i64,
    pub steps: i64,
    pub distance: i64,
    pub calories: i64,
    pub duration: i64,
    pub floors: i64,

}

impl Basket {
    pub fn new(
        start_month: i64, 
        total_steps: i64, 
        total_distance: i64, 
        total_calories: i64, 
        total_duration: i64, 
        total_floors: i64, 
        steps: i64, 
        distance: i64, 
        calories: i64, 
        duration: i64, 
        floors: i64) -> Basket {

        Basket {
            start_month,
            total_steps,
            total_distance,
            total_calories,
            total_duration,
            total_floors,
            steps,
            distance,
            calories,
            duration,
            floors,
        }
    }
}
    




















// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@






// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {

    // use super::*;
    // use std::{fs::copy};
    // use substring::Substring;
    // use std::fs::remove_file;

    
    // #[ignore]
    #[test]
    fn t001_new() {

        assert_eq!(4,4);
        
    }
    






















}//end of tests

















