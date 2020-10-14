use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    if s.len() == 1 {
        println!("{}", 0);
        return;
    }

    let mut v = vec![];
    let mut i = 0;
    while i < s.len() {
        if s[i] == '2' && i + 1 < s.len() && s[i + 1] == '5' {
            v.push(true);
            i += 2;
        } else {
            v.push(false);
            i += 1;
        }
    }

    let mut c = 0_i64;
    let mut sum = 0_i64;
    for &v_i in v.iter() {
        if v_i {
            c += 1;
            sum += c;
        } else {
            c = 0_i64;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
