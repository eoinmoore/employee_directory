use std::io;
use std::io::Write;
use std::collections::HashMap;

struct Directory {
   map: HashMap<String, Vec<String>>,
}

impl Directory {
    fn add_employee(&mut self, emp_name: String, dept_name: String) {
        println!("Adding {} to {}", emp_name, dept_name);
        let dept = self.map.entry(dept_name).or_insert(Vec::new());
        dept.push(emp_name);
    }

    fn list_employees(&self) {
        for (dept, emp_list) in &self.map {
            println!("{}:", dept);
            println!("----");
            for emp in emp_list {
                println!("  {}", emp);
            }
            println!();
        }
    }
}

fn main() {
    let mut dir = Directory{
        map: HashMap::new(),
    };
    println!("Welcome to the employee directory!");
    loop {
        print!(" >> ");
        io::stdout().flush().unwrap();
        let input = get_input();
        let mut command = Vec::new();
        for word in input.trim().split_whitespace() {
            command.push(word.to_string());
        }

        if command.len() == 0 {
            continue;
        }

        match command[0].to_lowercase().as_str() {
            "add" => {
                if command.len() != 4 || command[2].to_lowercase() != "to" {
                    invalid_command_message(&command);
                    continue;
                }
                command.remove(0);
                dir.add_employee(command[0].clone(), command[2].clone());
            },
            "list" => {
                if command.len() != 1 {
                    invalid_command_message(&command);
                    continue;
                }
                dir.list_employees();
            },
            "exit" => {
                println!("Have a nice day!");
                break
            },
            _ => {
                invalid_command_message(&command);
                continue;
            },
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input
}

fn invalid_command_message(command: &Vec<String>) {
    print!("Error: invalid command:");
    for word in command {
       print!(" {}", word);
    }
    println!();
    usage();
}

fn usage() {
    println!("\
Usage:
  add <employee> to <directory>: Add employee to directory
  list: List all previously added employees");
}
