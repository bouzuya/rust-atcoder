use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s
        .into_iter()
        .map(|c| match c {
            'v' => 1,
            'w' => 2,
            _ => unreachable!(),
        })
        .sum::<usize>();
    println!("{}", ans);
}
