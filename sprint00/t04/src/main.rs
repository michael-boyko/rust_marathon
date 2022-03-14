mod m_ref;

fn main() {
    let mut n: i32 = 0;
    println!("prev n = {}", n);
    m_ref::change_by_ref(&mut n);
    println!("after n = {}", n);
}
