use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut count = vec![0_usize; 8];
    for a_i in a {
        count[a_i] += 1;
    }
    let ans = count.into_iter().min().unwrap();
    println!("{}", ans);
}
