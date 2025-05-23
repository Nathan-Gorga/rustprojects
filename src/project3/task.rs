/*WHAT IS #[derive(Debug)]

derive asks the compiler to create a Debug trait

a debug trait helps for debugging using println!("{:?}", var);*/
#[derive(Debug)]
pub struct Task{
    pub description: String,
    pub completed: bool,
}


/*WHAT IS IMPL

used to associate functions (methods) tout a struct, enum or trait*/
impl Task{
    pub fn new(description: String) -> Self {
        Task {
            description,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

