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
            _ => {},
        }
    }
}

/*fn is_command_valid(command: &String) -> bool {
    let mut ret: bool = false;
    let parsed_command: Vec<&str> = command.split(' ').collect();

    //println!("{:?}", parsed_command);
    if parsed_command.len() < 3 {
        println!("Yes!!!!!!!!!!!!!");
    }

    return ret;
}*/
