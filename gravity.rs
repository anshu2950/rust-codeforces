
use std::io;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("read");
    let n = x.to_string().trim().parse().expect("Parse Err");
    let mut vc = Vec::<u32>::new();
    x = "".to_string();
    io::stdin().read_line(&mut x).expect("Read");
    let numbers: Vec<_> = x.split_whitespace().collect();
    for i in numbers {
        vc.push(i.trim().parse().expect("Parse"));
    }
    for i in 0..n-1 {
        for j in i+1..n {
            if vc[i] > vc[j] {
                let tmp = vc[i];
                vc[i] = vc[j];
                vc[j] = tmp;
            }
        }
    }
    for i in 0..n {
        print!("{} ", vc[i]);
    }
    println!("");
}