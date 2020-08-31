use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut s_p = s[0];
    let mut c = 1;
    for &s_i in s.iter().skip(1) {
        if s_p != s_i {
            print!("{}{}", s_p, c);
            c = 0;
        }
        s_p = s_i;
        c += 1;
    }
    println!("{}{}", s_p, c);
}
