use std::io::{self, stdin};



fn main() {
    println!("Welcome to the CLI calculator");

    let a = get_number("Enter first number :"); // why need to create a function for input
    let b = get_number("Enter second number :");
    let operator = get_string("Enter the operator (+, -, *, /) :");


    match calculate(a,b,&operator){
        Ok(result) => println!("Result : {result}"),
        Err(e) => println!("Error : {e}"),
    }

}

fn get_number(message: &str) -> f64{//why reference of str?
    loop{ // why loop?
        println!("{}",message);

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Reading error");//what is this?


        match input.trim().parse::<f64>(){
            Ok(n) => return n,
            Err(_) => println!("Invalid Entry, please enter a number"),
        }
    }
}


fn get_string(message: &str) -> String {
    println!("{}", message);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    input.trim().to_string()
}

fn calculate(a: f64, b:f64, op: &str) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0{
                Err("Divided by zero !".to_string())
            }else{
                Ok(a/b)
            }
        }
        _ => Err(format!("non recognized operator : {}",op)),//why exclamation?
    }
}