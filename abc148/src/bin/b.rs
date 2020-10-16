use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    };
    for (s_i, t_i) in s.iter().zip(t.iter()) {
        print!("{}{}", s_i, t_i);
    }
    println!();
}
