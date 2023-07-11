use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    };
    let n = s.len();
    for i in 0..n {
        if s == t {
            println!("{}", i);
            return;
        }
        s.rotate_right(1);
    }
    if s == t {
        println!("{}", n);
    } else {
        println!("-1");
    }
}
