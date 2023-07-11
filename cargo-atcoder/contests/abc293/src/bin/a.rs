use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    let n = s.len();
    for i in (0..n).step_by(2) {
        s.swap(i, i + 1);
    }
    println!("{}", s.iter().collect::<String>());
}
