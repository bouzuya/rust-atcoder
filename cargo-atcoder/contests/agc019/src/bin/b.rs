use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
    };
    let n = a.len();
    let mut count = vec![0_usize; 26];
    for a_i in a {
        let index = (a_i as u8 - 'a' as u8) as usize;
        count[index] += 1;
    }
    let mut sum = 0;
    for i in 0..26 {
        sum += count[i] * (count[i].saturating_sub(1)) / 2;
    }
    let ans = n * (n - 1) / 2 - sum + 1;
    println!("{}", ans);
}
