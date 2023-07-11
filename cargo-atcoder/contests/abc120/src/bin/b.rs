use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a: usize,
        b: usize,
        k: Usize1,
    };

    let mut ds = vec![];
    for x in 1..=a.max(b) {
        if (a % x == 0) && (b % x == 0) {
            ds.push(x);
        }
    }
    println!("{}", ds.iter().rev().nth(k).unwrap());
}
