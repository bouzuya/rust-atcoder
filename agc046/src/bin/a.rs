use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    for k in 1.. {
        if (k * x) % 360 == 0 {
            println!("{}", k);
            return;
        }
    }
}
