pub fn mead_song() {
    let mut count: i32 = 99;
    let bottles: String = "bottles".to_string();
    let bottle: String = "bottle".to_string();
    let mut bt: &String;

    while count > 0 {
        if count > 1 {
            bt = &bottles;
        } else {
            bt = &bottle;
        }
        println!("{} {} of mead on the wall, {} {} of mead.", count, bt, count, bt);
        count -= 1;
        if count > 0 {
            bt = &bottle;
        } else {
            break;
        }
        println!("Take one down and pass it around, {} {} of mead on the wall.\n", count, bt);
    }
    println!("Take it down and pass it around, no more bottles of mead on the wall.\n");
    println!("No more bottles of mead on the wall, no more bottles of mead.");
    println!("Go to the store and buy some more, 99 bottles of mead on the wall.");
}
