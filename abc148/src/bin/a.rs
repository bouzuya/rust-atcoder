use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    for x in 1..=3 {
        if x == a || x == b {
            continue;
        }
        println!("{}", x);
    }
}
