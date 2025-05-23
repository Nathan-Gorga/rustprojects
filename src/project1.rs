use std::io::stdin;



/*WHY THE &str

we need a reference otherwise the ownership of the message gets transfered and dropped at the end of the functions scope.
*/
fn get_number(message: &str) -> f64{
    loop{       
        /*WHY THE "!"?

        because println! is a macro, because it takes in a variable ammount of arguments

        macros in Rust are not the same as in C/C++. They actually generate safe code in Rust at compilation.  
         */
        println!("{}",message);

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Reading error");

        match input.trim().parse::<f64>(){
            Ok(n) => return n,
            Err(_) => println!("Invalid Entry, please enter a number"),
        }
    }
}



/*WHY NO RETURN?

if there is no ";" and the last line, same as return*/
fn get_string(message: &str) -> String {
    println!("{}", message);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Reading error");

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
        /*WHAT IS FORMAT!()?
        
        returns a string
        
        WHAT IS "_"? 
        
        in a match, _ is a wildcard (default in switch case)*/
        _ => Err(format!("non recognized operator : {}",op)),
    }
}



pub fn project1(){
    println!("Welcome to the CLI calculator");

    let a = get_number("Enter first number :"); // why need to create a function for input
    let b = get_number("Enter second number :");
    let operator = get_string("Enter the operator (+, -, *, /) :");

    match calculate(a,b,&operator){
        Ok(result) => println!("Result : {result}"),
        Err(e) => println!("Error : {e}"),
    }
}