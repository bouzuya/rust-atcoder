use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    for n in 1.. {
        if n * n * n * n == x {
            println!("{}", n);
            return;
        }
    }
}
