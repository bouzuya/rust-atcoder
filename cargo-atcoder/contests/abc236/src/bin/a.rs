use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        mut s: Chars,
        a: Usize1,
        b: Usize1,
    };
    s.swap(a, b);
    let ans = s.into_iter().collect::<String>();
    println!("{}", ans);
}
