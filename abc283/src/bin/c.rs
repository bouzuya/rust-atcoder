use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut count = 0_usize;
    let mut zero = false;
    for c in s {
        if c == '0' {
            if zero {
                zero = false;
            } else {
                zero = true;
                count += 1;
            }
        } else {
            zero = false;
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
