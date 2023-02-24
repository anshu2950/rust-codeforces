use std::io;

fn out(dir: String, st: String) {
    let keys: String = "qwertyuiopasdfghjk;zxcvbnm,./".to_string();
    let mut s: i32;
    let e: i32;
    let d: i32;
    if dir == "R" {
        s = (keys.len() - 1) as i32;
        d = -1;
        e = -1;
    } else {
        s = 0;
        d = 1;
        e = keys.len() as i32;
    }
    let vec = Vec::<char>::new();
    for i in st.chars() {
        loop {
            if s == e { break; }
            if st.chars().nth(s as usize) == Some(i) {
                vec.push(keys.chars().nth((s + d)as usize));
                // print!("{:?}", st.chars().nth((s + d) as usize));
            }
            s += d;
        }
    }
    println!("{:?}", vec);
}

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Read");
    let dir = x.trim().to_string();
    x = "".to_string();
    io::stdin().read_line(&mut x).expect("Read");
    x = x.trim().to_string();
    out(dir, x);
}