use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut h = vec![];
    for i in 0..n / 2 {
        match s[i].iter().find(|&&c| s[n - i - 1].contains(&c)) {
            None => {
                println!("-1");
                return;
            }
            Some(&c) => h.push(c),
        }
    }
    println!(
        "{}{}{}",
        h.iter().collect::<String>(),
        if n % 2 == 0 {
            "".to_string()
        } else {
            s[n / 2][0].to_string()
        },
        h.iter().rev().collect::<String>()
    );
}
