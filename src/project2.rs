use rand::Rng;

//what is usize?
fn generate_password(length: usize) -> String{
    //is b for binary?
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    abcdefghijklmnopqrstuvwxyz\
                    0123456789!@#$%^&*()_-+=<>?";

    let mut rng = rand::thread_rng();//what is thread_rng?

    let mut password = Vec::with_capacity(length);//what is with_capacity

    for _ in 0..length{//what is ..length
        let idx = rng.gen_range(0..charset.len());
        password.push(charset[idx] as char);
    }

    password.into_iter().collect()
}

pub fn project2(){
    let length = 12;
    let password = generate_password(length);

    println!("Password generated : {password}");
}