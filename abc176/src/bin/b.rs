use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    };
    let mut sum = 0_i64;
    for &n_i in n.iter() {
        let d = (n_i as usize - '0' as usize) as i64;
        sum += d;
    }
    let ans = sum % 9 == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
