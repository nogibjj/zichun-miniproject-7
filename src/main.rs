use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
struct Passenger {
    passenger_id: u32,
    survived: u8,
    pclass: u8,
    name: String,
    sex: String,
    age: Option<f32>, // Age might have missing values, so it's optional
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

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data/titanic.csv"; // Change this to your actual CSV file path
    let passengers = read_csv(file_path)?;

    println!("Total number of passengers: {}", passengers.len());

    let survivors: Vec<&Passenger> = passengers.iter().filter(|p| p.survived == 1).collect();
    println!("Number of survivors: {}", survivors.len());

    let total_age: f32 = passengers.iter()
        .filter_map(|p| p.age)
        .sum();
    let count_age: usize = passengers.iter()
        .filter(|p| p.age.is_some())
        .count();
    let average_age = total_age / count_age as f32;
    println!("Average age of passengers: {:.2}", average_age);

    Ok(())
}
