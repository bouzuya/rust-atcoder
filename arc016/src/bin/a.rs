use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    for i in 1..=n {
        if i != m {
            println!("{}", i);
            return;
        }
    }
}
