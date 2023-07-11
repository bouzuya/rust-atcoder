use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s_1: Chars,
        s_2: Chars,
    };
    let p = 1_000_000_007_usize;
    let mut count = 1_usize;
    let mut v; // true: |, false: =
    let mut i = 0;
    if s_1[i] == s_2[i] {
        count *= 3;
        v = true;
        i += 1;
    } else {
        count *= 6;
        v = false;
        i += 2;
    }
    while i < n {
        if s_1[i] == s_2[i] {
            if v {
                count *= 2;
                count %= p;
            } else {
                count *= 1;
                count %= p;
            }
            v = true;
            i += 1;
        } else {
            if v {
                count *= 2;
                count %= p;
            } else {
                count *= 3;
                count %= p;
            }
            v = false;
            i += 2;
        }
    }
    let ans = count;
    println!("{}", ans);
}
