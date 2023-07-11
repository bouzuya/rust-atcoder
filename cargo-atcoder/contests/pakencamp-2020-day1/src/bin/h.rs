// <https://atcoder.jp/contests/pakencamp-2020-day1/editorial/516>
use proconio::input;

fn main() {
    input! {
        t: usize,
        abc: [(u64, u64, u64); t],
    };
    for &(a, b, c) in abc.iter() {
        let ans = ((a & b) == b) && (((b & c) == b) || ((b & c) == 0)) && ((a | c) == a);
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
