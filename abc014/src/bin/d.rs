use proconio::input;
use proconio::marker::Usize1;

fn main() {
    // おそらく LCA だが解く気力がない。
    todo!();
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let ans = n - a.len();
    println!("{}", ans);
}
