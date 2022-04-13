use std::env;
use std::process;

struct City {
    name: String,
    stamina: i32,
    distance: i32,
}

fn main() {
    let mut argv: Vec<String> = env::args().collect();
    let mut cities: Vec<City> = Vec::new();

    if argv.len() < 2 {
        println!("usage: ./visitAll [[name,stamina,distance] ...]");
    } else {
        argv.remove(0);
        for item in argv {
            cities.push(pars_argv(&item));
        }
    }
    if !find_a_way(cities) {
        println!("Mission: Impossible");
    }
}

fn find_a_way(cities: Vec<City>) -> bool {
    let mut way: Vec<&String> = Vec::new();
    let size: usize = cities.len();
    let mut passed: i32 = 0;
    let mut find: bool = false;
    let mut stamina: i32 = 0;

    for i in 0..size {
        for j in i..size {
            stamina += cities[j].stamina - cities[j].distance;
            if stamina < 0 {
                break;
            }
            way.push(&cities[j].name);
            passed += 1;
        }
        for k in 0..i {
            stamina += cities[k].stamina - cities[k].distance;
            if stamina < 0 {
                break;
            }
            way.push(&cities[k].name);
            passed += 1;
        }
        if passed == size as i32 {
            find = true;
            break;
        }
        passed = 0;
        stamina = 0;
    }

    if find {
        for city in way {
            let m = cities.iter().position(|s| &s.name == city).unwrap();
            println!("{}. {}", m, city);
        }
    }

    return find;
}

fn pars_argv(item: &String) -> City {
    let mut stamina: i32 = 0;
    let mut distance: i32 = 0;
    let splited: Vec<&str> = item.split(",").collect();

    if splited.len() != 3 || splited[0].is_empty() {
        error_handler(&item);
    }
    match splited[1].parse::<i32>() {
        Ok(n) => stamina = n,
        Err(_) => error_handler(&item), 
    }

    match splited[2].parse::<i32>() {
        Ok(n) => distance = n,
        Err(_) => error_handler(&item), 
    }

    let city: City = City {
        name: splited[0].to_string(),
        stamina: stamina,
        distance: distance,
    };
    
    return city;
}

fn error_handler(item: &String) {
    println!("Argument {} is not valid", item);
    process::exit(0x0100);
}
