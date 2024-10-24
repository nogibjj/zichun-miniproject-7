use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Passenger {
    passenger_id: u32,
    survived: u8,
    pclass: u8,
    name: String,
    sex: String,
    age: Option<f32>,
    sib_sp: u8,
    parch: u8,
    ticket: String,
    fare: f32,
    cabin: Option<String>,
    embarked: Option<String>,
}

fn read_csv(file_path: &str) -> Result<Vec<Passenger>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(file);

    let mut passengers: Vec<Passenger> = Vec::new();
    for result in rdr.deserialize() {
        let record: Passenger = result?;
        passengers.push(record);
    }
    Ok(passengers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv() {
        let passengers = read_csv("tests/sample_data.csv").unwrap();

        // Test the number of passengers in the file
        assert_eq!(passengers.len(), 2);

        // Test if first passenger matches expected data
        let expected_first_passenger = Passenger {
            passenger_id: 1,
            survived: 1,
            pclass: 3,
            name: "John Doe".to_string(),
            sex: "male".to_string(),
            age: Some(22.0),
            sib_sp: 1,
            parch: 0,
            ticket: "A/5 21171".to_string(),
            fare: 7.25,
            cabin: None,
            embarked: Some("S".to_string()),
        };
        assert_eq!(passengers[0], expected_first_passenger);
    }
}
