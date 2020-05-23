use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes
    };
    if s.iter().find(|&&c| b'a' <= c && c <= b'z').is_some() {
        println!("error");
        return;
    }
    let mut ans: u64 = 0;
    for &c in s.iter().skip_while(|&&c| c == b'0') {
        let d = (c - b'0') as u64;
        ans = ans * 10 + d;
    }
    ans *= 2;
    println!("{}", ans);
}
