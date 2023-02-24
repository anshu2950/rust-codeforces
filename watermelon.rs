use std::io;

fn main(){
    let mut guess = String::new();

    io::stdin().read_line(&mut guess);
    let k: u32 = guess.trim().parse().expect("Parse");

    if k%2==0 {
        print!("YES")
    }
    else {
        print!("NO")
    }

}