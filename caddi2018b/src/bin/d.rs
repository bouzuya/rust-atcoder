// 解説 AC
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let ans = !a.iter().all(|&a_i| a_i % 2 == 0);
    println!("{}", if ans { "first" } else { "second" });
}
