use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Passenger {
    PassengerId: u32,
    Survived: u8,
    Pclass: u8,
    Name: String,
    Sex: String,
    Age: Option<f32>,
    SibSp: u8,
    Parch: u8,
    Ticket: String,
    Fare: f32,
    Cabin: Option<String>,
    Embarked: Option<String>,
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
            PassengerId: 1,
            Survived: 1,
            Pclass: 3,
            Name: "John Doe".to_string(),
            Sex: "male".to_string(),
            Age: Some(22.0),
            SibSp: 1,
            Parch: 0,
            Ticket: "A/5 21171".to_string(),
            Fare: 7.25,
            Cabin: None,
            Embarked: Some("S".to_string()),
        };
        assert_eq!(passengers[0], expected_first_passenger);
    }
}
