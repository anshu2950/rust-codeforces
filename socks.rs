use std::io;
// 9 + 3 + 1
// 9 + out(3, 3) => 9 + 3 + out(1, 3) => 9 + 3 + 
// 10 + 5 + 2 + 1
fn out(a: u32, b: u32) -> u32 {
    return a + if a < b { 0 } else { out(a / b, b) };
}

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Read");
    let mut arr = Vec::<u32>::new();
    let v: Vec<_> = x.trim().split_whitespace().collect();
    for i in v {
        arr.push(i.trim().parse().expect("Parse"));
    }
    // let mut ans: u32 = 0;
    // let mut cnt: u32 = 0;
    // while arr[0] != 0 {
    //     arr[0] -= 1;
    //     ans += 1;
    //     cnt += 1;
    //     if cnt == arr[1] {
    //         cnt = 0;
    //         arr[0] += 1;
    //     }
    // }
    println!("{}", out(arr[0], arr[1]));
}