use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: u8,
        s: Chars,
    };
    for &s_i in s.iter() {
        let ci = s_i as u8 - 'A' as u8; // 0..26
        let cj = ((ci + n) % 26 + 'A' as u8) as char;
        print!("{}", cj);
    }
    println!();
}
