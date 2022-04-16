use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    for i in 0..=9 {
        let c = (b'0' + i) as char;
        if s.contains(&c) {
            continue;
        } else {
            println!("{}", c);
            return;
        }
    }
}
