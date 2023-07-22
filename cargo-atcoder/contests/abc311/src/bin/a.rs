use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut ok = vec![false; 3];
    for (i, s_i) in s.iter().copied().enumerate() {
        ok[(s_i as u8 - b'A') as usize] = true;
        if ok.iter().all(|&ok_i| ok_i) {
            println!("{}", i + 1);
            return;
        }
    }
}
