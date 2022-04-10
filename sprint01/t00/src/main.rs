use std::env;

fn main() {
    let mut argv: Vec<String> = env::args().collect();

    if argv.len() != 6 {
        println!("usage: ./simpleSort arg1 arg2 arg3 arg4 arg5");
    } else {
        argv.remove(0);
        argv.sort();
        println!("{:?}", argv);
    }
}
