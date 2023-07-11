use proconio::input;
use proconio::marker::Chars;

fn is_acgt(c: char) -> bool {
    match c {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

fn main() {
    input! {
        s: Chars
    };
    let mut max_len = 0;
    for i in 0..s.len() {
        let mut c = 0;
        for j in i..s.len() {
            if !is_acgt(s[j]) {
                break;
            }
            c += 1;
        }
        max_len = std::cmp::max(max_len, c);
    }
    let ans = max_len;
    println!("{}", ans);
}
