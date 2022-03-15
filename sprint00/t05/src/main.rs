use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    
    if argv.len() != 5 {
        println!("usage:./t05 [name] [level] [health] [stamina]");
    } else {
        let name = &argv[1];
        let level = argv[2].parse::<i32>().unwrap_or_else(|_| panic!("Invalid argument: {}", argv[2]));
        let health = argv[3].parse::<f32>().unwrap_or_else(|_| panic!("Invalid argument: {}", argv[3]));
        let stamina = argv[4].parse::<f32>().unwrap_or_else(|_| panic!("Invalid argument: {}", argv[4]));

        println!("Name = {}", *name);
        println!("Level = {}", level);
        println!("Health = {}", health);
        println!("Stamina = {}", stamina);
    }
}
