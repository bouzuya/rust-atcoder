use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        mut k: usize,
    };
    for c in s {
        match c {
            '1' => {
                k -= 1;
                if k == 0 {
                    println!("{}", c);
                    return;
                }
            }
            _ => {
                println!("{}", c);
                return;
            }
        }
    }
    unreachable!();
}
