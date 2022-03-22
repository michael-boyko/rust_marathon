use std::io;
use std::io::Write;
use std::process;
use std::collections::LinkedList;

fn main() {
    let mut inventory: LinkedList<char> = LinkedList::from(['-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-',]);

    loop {
        let mut input = String::new();
        print!("Enter command:> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Can not read line!");
        let input: Vec<&str> = input.split(' ').collect();

        if input.len() > 2 {
            println!("Invalid command.");
        } else {
            command_handler(input, &mut inventory);
        }
    }
}

fn command_handler(commands: Vec<&str>, inv: &mut LinkedList<char>) {
    if commands.len() == 2 {
        let command = commands[0].trim();
        let item = commands[1].trim();
        match command {
            "insert" => insert_item(inv, &item),
            "remove" => remove_item(inv, &item),
            _ => println!("Invalid command."),
        }
    } else {
        let command = commands[0].trim();
        match command {
            "exit" => exit_command(),
            "help" => print_help(),
            "show" => println!("Inventory {:?}", inv),
            _ => println!("Invalid command."),
        }
    }
}

fn insert_item(inv: &mut LinkedList<char>, item: &str) {
    let targets: String = String::from("a f p w");
    let ch: char = item.chars().next().unwrap();
    let mut is_full: bool = true;

    if targets.contains(item) {
        for one in inv {
            if *one == '-' {
                *one = ch;
                is_full = false;
                println!("{} was inserted.", ch);
                break;
            }
        }
        if is_full == true {
            println!("Inventory is full.");
        }
    } else {
        println!("Invalid item.");
    }
}

fn remove_item(inv: &mut LinkedList<char>, item: &str) {
    let targets: String = String::from("a f p w");
    let ch: char = item.chars().next().unwrap();
    
    if targets.contains(item) {
        if inv.contains(&ch) {
            for one in inv {
                if *one == ch {
                    *one = '-';
                    println!("{} was removed.", ch);
                    break;
                }
            }
        } else {
            println!("Invalid item.");
        }
    } else {
        println!("Invalid item.");
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

fn exit_command() {
    println!("Bye.");
    process::exit(0x0100);
}
