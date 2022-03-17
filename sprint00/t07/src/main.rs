use std::io;
use std::io::Write;
use std::process;

fn main() {
    loop {
        let mut input = String::new();
        print!("Enter command:> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Can not read line!");
        let input: Vec<&str> = input.split(' ').collect();

        if input.len() > 2 {
            println!("Invalid command.");
        } else {
            command_handler(input);
        }
        //println!("Len = {}, str= {}", input.len(), input);
        //is_command_valid(&input);
    }
    //println!("You put {}", input);
}

fn command_handler(commands: Vec<&str>) {
    if commands.len() == 2 {
println!("2");
    } else {
        let command = commands[0].trim();
        match command {
            "exit" => process::exit(0x0100),
            "help" => print_help(),
            _ => {},
        }
    }
}

fn print_help() {
    println!("Available commands:");
    println!("1. insert <itemType>");
    println!("2. remove <itemType>");
    println!("3. show");
    println!("4. help");
    println!("5. exit");
}
