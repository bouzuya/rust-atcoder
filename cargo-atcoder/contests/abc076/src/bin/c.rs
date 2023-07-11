use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let index = s
        .windows(t.len())
        .enumerate()
        .filter(|(_, s)| {
            s.iter()
                .zip(t.iter())
                .all(|(&c1, &c2)| c1 == '?' || c1 == c2)
        })
        .map(|(i, _)| i)
        .max();
    match index {
        None => println!("UNRESTORABLE"),
        Some(i) => {
            let ans = s[..i]
                .iter()
                .chain(t.iter())
                .chain(s[i + t.len()..].iter())
                .map(|&c| if c == '?' { 'a' } else { c })
                .collect::<String>();
            println!("{}", ans);
        }
    }
}
