use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let ans = s
        .into_iter()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => unreachable!(),
        })
        .collect::<String>();
    println!("{}", ans);
}
