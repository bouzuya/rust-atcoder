use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [Usize1; m],
    };

    let l: usize = x[0];
    let r: usize = n - 1 - x[m - 1];

    todo!();

    let ans = n - a.len();
    println!("{}", ans);
}
