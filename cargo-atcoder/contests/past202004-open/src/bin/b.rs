use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut count = vec![0_usize; 3];
    for c in s {
        count[(c as u8 - b'a') as usize] += 1;
    }
    let max = count.iter().copied().max().unwrap();
    let ans = (count.iter().copied().position(|v| v == max).unwrap() as u8 + b'a') as char;
    println!("{}", ans);
}
