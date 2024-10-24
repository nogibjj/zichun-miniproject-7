use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
struct Passenger {
    PassengerId: u32,
    Survived: u8,
    Pclass: u8,
    Name: String,
    Sex: String,
    Age: Option<f32>, // Age might have missing values, so it's optional
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

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data/titanic.csv"; 
    let passengers = read_csv(file_path)?;

    println!("Total number of passengers: {}", passengers.len());

    let survivors: Vec<&Passenger> = passengers.iter().filter(|p| p.Survived == 1).collect();
    println!("Number of survivors: {}", survivors.len());

    let total_age: f32 = passengers.iter()
        .filter_map(|p| p.Age)
        .sum();
    let count_age: usize = passengers.iter()
        .filter(|p| p.Age.is_some())
        .count();
    let average_age = total_age / count_age as f32;
    println!("Average age of passengers: {:.2}", average_age);

    Ok(())
}
