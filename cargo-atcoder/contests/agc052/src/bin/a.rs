use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            _: [String; 3],
        }
        println!("{}{}0", "0".repeat(n), "1".repeat(n));
    }
}
