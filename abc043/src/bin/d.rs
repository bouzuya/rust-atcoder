use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut is = vec![None; 26];
    for (i, &c) in s.iter().enumerate() {
        let index = (c as u8 - 'a' as u8) as usize;
        if let Some(p) = is[index] {
            if i - p <= 2 {
                println!("{} {}", p + 1, i + 1);
                return;
            }
        }
        is[index] = Some(i);
    }
    println!("-1 -1");
}
