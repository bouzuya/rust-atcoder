use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    for x in 1..=127 {
        if x % 3 == a && x % 5 == b && x % 7 == c {
            println!("{}", x);
        }
    }
}
