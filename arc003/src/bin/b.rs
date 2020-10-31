use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    };
    s.sort_by_key(|s_i| {
        let mut t = s_i.clone();
        t.reverse();
        t
    });
    for s_i in s {
        println!("{}", s_i.iter().collect::<String>());
    }
}
