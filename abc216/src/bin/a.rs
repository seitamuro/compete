use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        x: f32,
    }

    let y = (x*10. % 10.) as usize;
    let x = x as usize;

    if y <= 2 {
        println!("{}-", x);
    } else if 3 <= y && y <= 6 {
        println!("{}", x);
    } else {
        println!("{}+", x);
    }
}
