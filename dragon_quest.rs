use std::io;
/**
 * 7
 * 1000 3 4
 * 123 4 2
 * ...
 */
fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Read");
    let t: u16 = x.trim().parse().expect("Parse");
    // let mut arr: Vec<_>;
    for _p in 0..t {
        let mut x = String::new();
        let mut suc: bool = false;
        let arr: Vec<_>;
        io::stdin().read_line(&mut x).expect("Read");
        arr = x.split_whitespace().collect();
        let mut vec = Vec::<i32>::new();
        for i in arr {
            vec.push(i.trim().parse().expect("Parse"));
        }
        loop {
            if vec[0] <= 0 {
                // println!("{:?}", vec);
                suc = true;
                break;
            }
            if vec[0] >= 20 {
                if vec[1] != 0 {
                    vec[0] = (vec[0] / 2) + 10;
                    vec[1] -= 1;
                } else if vec[2] != 0 {
                    vec[0] -= 10;
                    vec[2] -= 1;
                } else {
                    break;
                }
            } else {
                if vec[2] != 0 {
                    vec[0] -= 10;
                    vec[2] -= 1;
                } else {
                    break;
                }
            }
            // println!("{}", vec[0]);
        }
        println!("{}", if suc {"YES"} else {"NO"});
        // println!("{}", vec[0]);
        // println!("{:?}", vec);
    }
}