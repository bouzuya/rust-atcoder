use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: Usize1,
    };
    let mut sum = 0;
    for i in a..=n * k + a {
        sum += 1;
        if sum >= k {
            println!("{}", i % n + 1);
            break;
        }
    }
}
