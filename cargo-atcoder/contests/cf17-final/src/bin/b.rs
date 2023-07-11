use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let mut count = vec![0; 3];
    for s_i in s {
        let index = (s_i as u8 - 'a' as u8) as usize;
        count[index] += 1;
    }
    let ans = count.iter().filter(|&&c| c == n / 3).count() == 3 - n % 3
        && count.iter().filter(|&&c| c == n / 3 + 1).count() == n % 3;
    println!("{}", if ans { "YES" } else { "NO" });
}
