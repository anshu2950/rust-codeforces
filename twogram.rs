use std::io;
use std::collections::HashMap;

fn main() {
    let mut x = String::new();
    let mut twograms = HashMap::new();
    io::stdin().read_line(&mut x).expect("Read");
    x = "".to_string();
    io::stdin().read_line(&mut x).expect("Read");
    let mut ans = String::new();
    let mut max = 0;
    for i in 0..(x.len()-2) {
        let uval:u32;
        // twograms.entry(&x[i..i+2]).or_insert(0);
        uval = match twograms.get(&x[i..i+2]) {
            Some(x) => { *x as u32 },
            None => { 0 },
        };

        twograms.insert(&x[i..i+2], uval + 1);
        if uval + 1 > max {
            max = uval + 1;
            ans = String::from(&x[i..i+2]);
        }
    }
    println!("{}", ans);
    // println!("{}", &x[0..20]);
}