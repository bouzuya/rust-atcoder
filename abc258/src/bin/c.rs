use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        query: [(usize, usize); q],
    };
    let mut index = 0_usize;
    for (t, x) in query {
        match t {
            1 => {
                index = (n + index - x) % n;
            }
            2 => {
                println!("{}", s[(index + x - 1) % n]);
            }
            _ => unreachable!(),
        }
    }
}
