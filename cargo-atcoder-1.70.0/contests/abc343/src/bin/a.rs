use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    for i in 0..=9 {
        if i != a + b {
            println!("{}", i);
            return;
        }
    }
}
