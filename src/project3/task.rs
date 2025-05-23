#[derive(Debug)]
pub struct Task{
    pub description: String,
    pub completed: bool,
}

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

