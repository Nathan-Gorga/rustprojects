mod stats;


use csv::Reader;
use serde::Deserialize;
use stats::{calculate_mean, calculate_median};


#[derive(Debug, Deserialize)]
struct Record{
    value: f64,
}
/*WHAT IS BOX?

smart pointer (std::unique_ptr)*/
pub fn project4(path: String) -> Result<(), Box<dyn std::error::Error>>{
    let mut rdr = Reader::from_path(path)?;
    let mut values = Vec::new();


    for result in rdr.deserialize(){
        let record: Record = result?;
        values.push(record.value);
    }

    println!("Nombre de lignes : {}", values.len());

    if let Some(mean) = calculate_mean(&values){
        println!("Moyenne des valeurs : {:.2}",mean);
    }

    if let Some(median) = calculate_median(&mut values) {
        println!("Médiane des valeurs : {:.2}", median);
    }

    Ok(())
    
}