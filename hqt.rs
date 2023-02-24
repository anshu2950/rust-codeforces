use std::io;

fn main() {
    let mut str = String::new();
    io::stdin().read_line(& mut str).expect("Read");
    str = str.trim().to_string();

    for i in str.chars() {
        if i == 'H' || i == 'Q' || i == '9' {
            println!("Yes");
            return;
        } 
    }
    println!("No");
}