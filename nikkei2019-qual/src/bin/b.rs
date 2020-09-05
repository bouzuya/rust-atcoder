use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        a: Chars,
        b: Chars,
        c: Chars,
    };
    let mut count = 0;
    for ((a_i, b_i), c_i) in a.iter().zip(b.iter()).zip(c.iter()) {
        if a_i == b_i && b_i == c_i {
            // do nothing
        } else if a_i == b_i || b_i == c_i || a_i == c_i {
            count += 1;
        } else {
            count += 2;
        }
    }
    let ans = count;
    println!("{}", ans);
}
