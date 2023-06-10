use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    for d in 0..=5 {
        if (n + d) % 5 == 0 {
            println!("{}", n + d);
            return;
        }
        if (n - d) % 5 == 0 {
            println!("{}", n - d);
            return;
        }
    }
}
