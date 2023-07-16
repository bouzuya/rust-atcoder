use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s
        .iter()
        .copied()
        .rev()
        .map(|s_i| match s_i {
            '0' => '0',
            '1' => '1',
            '6' => '9',
            '8' => '8',
            '9' => '6',
            _ => unreachable!(),
        })
        .collect::<String>();
    println!("{}", ans);
}
