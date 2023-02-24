use std::io;

fn main() {
    let mut str = String::new();
    io::stdin().read_line(& mut str).expect("read");
    str = str.trim().to_string();
    let mut cnt = 0;
    let mut curr = 'x';
    for i in str.chars() {
        if curr != i {
            cnt = 1;
            curr = i;
        } else {
            cnt += 1;
            if cnt == 7 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
    // println!("{0}", str);
}