use std::io;
use std::cmp;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Read");
    buf = "".to_string();
    io::stdin().read_line(&mut buf).expect("Read");
    let s_arr : Vec<_> = buf.split_whitespace().collect();
    let mut curr_max = u32::MAX;
    let mut curr_score = 0;
    let mut max_score = 0;
    for i in s_arr {
        let x = i.trim().parse().expect("Parse");
        if x < curr_max {
            max_score = cmp::max(max_score, curr_score); // ? max_score : curr_score;
            curr_score = 0;
        }
        curr_max = x;
        curr_score += 1;
    }
    println!("{}", cmp::max(curr_score, max_score));
}