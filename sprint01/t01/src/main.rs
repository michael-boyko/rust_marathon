use std::env;

fn main() {
    let mut argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        println!("usage: ./moveAlong [args]");
    } else {
        argv.remove(0);
        for item in argv {
            let lower = item.to_lowercase();
            if lower.contains("mercer") ||
               lower.contains("emer") || lower.contains("jim") {
                println!("{}, move along!", item);
            }
        }
    }

}
