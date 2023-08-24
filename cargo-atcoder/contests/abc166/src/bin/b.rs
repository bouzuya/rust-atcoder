use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut snuke = vec![false; n];
    for _ in 0..k {
        input! {
            d: usize,
            a: [Usize1; d],
        };
        for a_i in a {
            snuke[a_i] = true;
        }
    }
    let ans = snuke.into_iter().filter(|b| !b).count();
    println!("{}", ans);
}
