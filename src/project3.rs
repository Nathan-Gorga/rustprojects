mod task;
use task::Task;

use std::io::{self, Write};


pub fn project3(){
    let mut tasks: Vec<Task> = Vec::new();

    loop{
        println!("\n--- TO-DO MENU ---");
        println!("1. Add a task");
        println!("2. Print tasks");
        println!("3. Mark as done");
        println!("4. Quit");

        print!("Choice : ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice{
            "1" => {
                print!("Description de la tâche : ");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                tasks.push(Task::new(desc.trim().to_string()));
            }
            "2" => {
                println!("\n--- Liste des tâches ---");
                for (i, task) in tasks.iter().enumerate(){
                    let status = if task.completed {"✔️"} else {"❌"};
                    println!("{}: [{}] {}", i+1, status, task.description);
                }
            }
            "3" => {
                print!("Numéro de la tâche à terminer : ");
                io::stdout().flush().unwrap();
                let mut index = String::new();

                io::stdin().read_line(&mut index).unwrap();

                if let Ok(i) = index.trim().parse::<usize>(){
                    if let Some(task) = tasks.get_mut(i-1){
                        task.complete();
                    }else{
                        println!("Tâche introuvable.");
                    }
                }else{
                    println!("Entrée invalide");
                }
            }
            "4" => break,
            _ => println!("Option invalide"),
        }
    }
    println!("À bientot !");

}