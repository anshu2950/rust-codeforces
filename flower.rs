use std::io;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Read");
    let k: u32 = x.trim().parse().expect("Parse");
    x = "".to_string();
    io::stdin().read_line(&mut x).expect("Read");
    let vec: Vec<_> = x.split_whitespace().collect();
    let mut arr = Vec::<u32>::new();
    for i in vec {
        arr.push(i.trim().parse().expect("Parse"));
    }
    arr.sort_by(|a, b| b.cmp(a));
    let mut sum: u32 = 0;
    let mut ans: i32 = 0;
    for i in arr {
        if sum >= k {
           break;
        }
        sum += i;
        ans += 1;
    }
    println!("{}", if sum < k { -1 } else { ans });
}