mod print_dialog;

fn main() {
    print_dialog::print_dialog(&String::from("Guard"), &String::from("I used to be an adventurer like you. Then I took an arrow in the knee..."));
    print_dialog::print_dialog(&String::from("Michael"), &String::from("How are you?"));
    print_dialog::print_dialog(&String::from("Kamilla"), &String::from("I'm fine, thx!"));
}
