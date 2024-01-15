use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };
    if n == 1 {
        println!("{}", a[0][0]);
        return;
    }

    let b = a
        .into_iter()
        .map(|a_i| a_i.into_iter().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();
    let mut c = vec![];
    for i in 0..n / 2 + if n % 2 == 0 { 0 } else { 1 } {
        let j = n - i - 1;
        if let Some(x) = b[i].intersection(&b[j]).next() {
            c.push(*x);
        } else {
            println!("-1");
            return;
        }
    }
    let mut ans = c.clone();
    if n % 2 == 0 {
        c.reverse();
        ans.extend(c);
    } else {
        let l = c.len().saturating_sub(1);
        ans.extend(c.into_iter().take(l).rev());
    }
    println!("{}", ans.iter().collect::<String>());
}
