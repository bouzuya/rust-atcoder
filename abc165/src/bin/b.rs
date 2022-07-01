use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let mut y = 100;
    for i in 1.. {
        y += y / 100;
        if y >= x {
            println!("{}", i);
            break;
        }
    }
}
