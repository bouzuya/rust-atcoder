use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; 4 * n - 1],
    };
    let mut count = vec![0_usize; n];
    for a_i in a {
        count[a_i] += 1;
    }
    let ans = count.iter().position(|i| *i != 4).unwrap() + 1;
    println!("{}", ans);
}
