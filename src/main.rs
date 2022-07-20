use std::collections::HashMap;

fn main() {
    let mut database: HashMap<_, _> = HashMap::new();

    loop {
        let mut user_input = String::new();

        println!(
            "\nWhat operation you want to perform?
    
            '1' -> to exit the program
            '2'-> to add student  
            '3' -> to show the database\n"
        );

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read the user's input!");

        if user_input.trim() == "1" {
            break;
        }
        else if user_input.trim() == "2" {
            let mut name = String::new();

            println!("\nEnter the name of the student:");
            std::io::stdin()
                .read_line(&mut name)
                .expect("Duh!");

            let mut department = String::new();

            println!("\nEnter the name of the department:");
            std::io::stdin().read_line(&mut department).expect("Duh!");

            database.insert(name.trim().to_string(), department.trim().to_string());

            println!("\nAdded {} to {} department.\n", name.trim(), department.trim());


        } else if user_input.trim() == "3" {
            println!("\n{:?}\n", database);
        }
    }
}
