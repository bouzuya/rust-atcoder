use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        l: usize,
        r: usize,
    };
    if s[0] == '0' && s.len() != 1 {
        println!("No");
        return;
    }
    let mut x = 0_usize;
    for s_i in s.iter().copied() {
        let d = (s_i as u8 - b'0') as usize;
        match x.checked_mul(10).and_then(|x| x.checked_add(d)) {
            None => {
                println!("No");
                return;
            }
            Some(next) => {
                x = next;
            }
        }
    }
    let ans = (l..=r).contains(&x);
    println!("{}", if ans { "Yes" } else { "No" });
}
