use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    s.reverse();
    let mut t = vec![];
    for c in s {
        t.push(match c {
            '0' => '0',
            '1' => '1',
            '6' => '9',
            '8' => '8',
            '9' => '6',
            _ => unreachable!(),
        });
    }
    let ans = t.iter().collect::<String>();
    println!("{}", ans);
}
