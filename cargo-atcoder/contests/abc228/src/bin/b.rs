use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        a: [Usize1; n],
    };
    let mut used = vec![false; n];
    let mut i = x;
    while !used[i] {
        used[i] = true;
        i = a[i];
    }
    let ans = used.iter().filter(|&&b| b).count();
    println!("{}", ans);
}
