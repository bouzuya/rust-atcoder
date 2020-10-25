use proconio::input;
use proconio::marker::Chars;

fn main() {
    use std::collections::HashSet;
    input! {
        n: usize,
        a: [Chars; n],
    };
    if n == 1 {
        println!("{}", a[0][0]);
        return;
    }
    let sets = a
        .into_iter()
        .map(|a_i| a_i.into_iter().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();
    let mut half = vec![];
    for i in 0..n / 2 {
        match sets[i].intersection(&sets[n - 1 - i]).next() {
            None => {
                println!("-1");
                return;
            }
            Some(c) => half.push(c),
        }
    }
    println!(
        "{}{}{}",
        half.iter().map(|&c| c).collect::<String>(),
        if n % 2 == 0 {
            "".to_owned()
        } else {
            sets[n / 2].iter().next().unwrap().to_string()
        },
        half.iter().rev().map(|&c| c).collect::<String>(),
    );
}
