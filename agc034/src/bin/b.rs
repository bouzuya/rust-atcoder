use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    let n = s.len();
    let mut t = vec![];
    let mut i = 0;
    while i < n {
        if s[i] == 'B' && i + 1 < n && s[i + 1] == 'C' {
            t.push('D');
            i += 2;
        } else {
            t.push(s[i]);
            i += 1;
        };
    }
    let mut c = 0_i64;
    let mut c_a = 0_i64;
    for &t_i in t.iter() {
        match t_i {
            'A' => c_a += 1,
            'D' => c += c_a,
            _ => c_a = 0,
        }
    }
    println!("{}", c);
}
