use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    };
    let ans = match x.iter().position(|c| c == &'.') {
        None => x.into_iter().collect::<String>(),
        Some(i) => x[0..i].iter().collect::<String>(),
    };
    println!("{}", ans);
}
