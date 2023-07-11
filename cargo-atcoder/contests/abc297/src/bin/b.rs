use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut bs = vec![];
    for i in 0..s.len() {
        if s[i] == 'B' {
            bs.push(i);
        }
    }
    if bs[0] % 2 == bs[1] % 2 {
        println!("No");
        return;
    }

    let mut rs = vec![];
    for i in 0..s.len() {
        if s[i] == 'R' {
            rs.push(i);
        }
    }
    let k = s.iter().position(|c| c == &'K').unwrap();
    if (rs[0]..=rs[1]).contains(&k) {
        println!("Yes");
        return;
    }

    println!("No");
}
