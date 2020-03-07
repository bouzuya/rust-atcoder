use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    for x in 1..=100 * 100 {
        if x * 8 / 100 == a && x * 10 / 100 == b {
            println!("{}", x);
            return;
        }
    }
    println!("{}", -1);
}
