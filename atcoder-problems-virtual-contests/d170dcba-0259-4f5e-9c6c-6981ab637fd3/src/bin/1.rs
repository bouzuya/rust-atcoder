use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    for c in 0..=255 {
        if a ^ b == c {
            println!("{}", c);
            return;
        }
    }
}
