// mod project1;
// use project1::*;

// mod project2;
// use project2::*;

// mod project3;
// use project3::*;

mod project4;
use project4::*;

fn main(){
    //project1();
    // project2();
    // project3();

    let path = String::from("./project4/data.csv");
    let _ = project4(path);
}
