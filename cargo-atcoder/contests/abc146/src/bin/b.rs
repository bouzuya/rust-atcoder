use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        s: Bytes,
    };
    let ans = s
        .iter()
        .copied()
        .map(|s_i| ((((s_i - b'A') as usize + n) % 26) as u8 + b'A') as char)
        .collect::<String>();
    println!("{}", ans);
}
