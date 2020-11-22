use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        x: i64,
        s: Chars,
    };
    let mut sum = x;
    for s_i in s {
        if s_i == 'o' {
            sum += 1
        } else {
            sum -= 1;
            if sum < 0 {
                sum = 0;
            }
        }
    }
    let ans = sum;
    println!("{}", ans);
}
