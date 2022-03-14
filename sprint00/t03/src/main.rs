mod cast;

fn main() {
    let f: f32 = 3.2;
    println!("Float {} to int {}", f, cast::float_to_int(f));
}
