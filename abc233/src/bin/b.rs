use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        l: Usize1,
        r: usize,
        s: Chars,
    };
    let n = s.len();
    if l != 0 {
        print!("{}", s[0..l].iter().collect::<String>());
    }
    print!("{}", s[l..r].iter().rev().collect::<String>());
    if r != n {
        print!("{}", s[r..n].iter().collect::<String>());
    }
}
