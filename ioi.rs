use std::io;
use std::cmp;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Read");
    let grid: Vec<_> = x.split_whitespace().collect();
    let n: u32 = grid[0].trim().parse().expect("Parse");
    let m: u32 = grid[1].trim().parse().expect("Parse");
    let p = cmp::min(n, m);
    println!("{}", if p & 1 == 1 { "Akshat" } else { "Malvika" });
}