use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut c = vec![0; n];
    for a_i in a {
        c[a_i] += 1;
    }
    let ans = c.iter().all(|&c_i| c_i == 1);
    println!("{}", if ans { "Yes" } else { "No" });
}
