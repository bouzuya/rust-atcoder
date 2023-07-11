use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        k: usize,
    };
    let mut t = vec![];
    let mut c = k;
    for &s_i in s.iter() {
        let d = s_i as usize - 'a' as usize; // 'a': 0, 'b': 1, 'c': 2, 'z': 25
        if d == 0 {
            t.push(s_i);
        } else {
            t.push(if 26 <= c + d {
                c -= 26 - d;
                'a'
            } else {
                s_i
            });
        }
    }
    if c > 0 {
        let n = t.len();
        let s_n = t[n - 1];
        let d = s_n as usize - 'a' as usize;
        t[n - 1] = (((d + c) % 26) as u8 + 'a' as u8) as char;
    }
    println!("{}", t.iter().collect::<String>());
}
