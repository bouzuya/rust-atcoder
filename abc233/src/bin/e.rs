use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    };
    let n = x.len();
    let mut s = vec![0_usize; n];
    for (i, x_i) in x.iter().copied().enumerate() {
        s[i] = (x_i as u8 - b'0') as usize + if i > 0 { s[i - 1] } else { 0 };
    }
    let mut c = 0_usize;
    let mut ans = vec![];
    for (_, s_i) in s.iter().copied().enumerate().rev() {
        ans.push((s_i + c) % 10);
        c = (s_i + c) / 10;
    }
    if c != 0 {
        ans.push(c);
    }
    ans.reverse();
    println!(
        "{}",
        ans.into_iter()
            .map(|c| (c as u8 + b'0') as char)
            .collect::<String>()
    );
}
