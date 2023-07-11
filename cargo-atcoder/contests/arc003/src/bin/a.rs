use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        r: Chars,
    };
    let mut sum = 0;
    for r_i in r {
        sum += match r_i {
            'A' => 4,
            'B' => 3,
            'C' => 2,
            'D' => 1,
            'F' => 0,
            _ => unreachable!(),
        };
    }
    let ans = sum as f64 / n as f64;
    println!("{}", ans);
}
