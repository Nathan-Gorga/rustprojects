use rand::Rng;

/*WHAT IS USIZE?

unsigned pointer size, used for indexing and sizes to stay consistent with the OS's pointer size*/
fn generate_password(length: usize) -> String{
    /*WHAT IS b"..."
    
    Creates an array of bytes, not a unicode string*/
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                       abcdefghijklmnopqrstuvwxyz\
                                       0123456789!@#$%^&*()_-+=<>?";

    /*WHAT IS THREAD_RNG()

    RNG on your thread, does not block the others*/
    let mut rng = rand::thread_rng();

    /*WHAT IS WITH_CAPACITY()
    
    creates an empty vector with memory space of length*/
    let mut password = Vec::with_capacity(length);

    /*WHY USE _
    
    When we don't care about the value of the index and just want a repeat loop*/
    for _ in 0..length{
        let idx = rng.gen_range(0..charset.len());
        password.push(charset[idx] as char);
    }

    /*WHAT IS INTO_ITER()?
    
    gives each element of the chain one by one (can be used in a loop)
    
    it also consumes the object after the function
    
    WHAT IS COLLECT()?

        Vec → String

        Iterator → Vec

        Iterator → HashMap, etc.
    */

    password.into_iter().collect()
}

pub fn project2(){
    let length = 12;
    let password = generate_password(length);

    println!("Password generated : {password}");
}