/*
    Most of my functions are in here.
    2022.11.26      Sven Ponelat

*/

fn parse_field<T: std::str::FromStr>(field_str: &str, field_name: &str, line: i64 ) -> Result<T, String> {
    match field_str.parse::<T>() {
        Ok(val) => Ok(val),
        Err(err) => {
            println!("Could not parse {}: {} in line number {}", field_name, err, line);
            Err(err)
        }
    }
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
    fn t001_parse() {
        let line: i64 = 12303;
        let field: String = "date";
        let value: String = "2022-12-05 08:00:00 +0000";


        

        let two = &res_conversion[2];
        assert_eq!(two.date,1537401600);
        assert_eq!(two.steps,6958);
        assert_eq!(two.duration,4356);
        assert_eq!(two.distance,4202);
        assert_eq!(two.calories,377);
        assert_eq!(two.floors,12);
    }











} // end of tests