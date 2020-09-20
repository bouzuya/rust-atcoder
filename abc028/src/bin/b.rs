use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    let mut c = vec![0; 6];
    for &s_i in s.iter() {
        let i = s_i as usize - 'A' as usize;
        c[i] += 1;
    }
    println!("{} {} {} {} {} {}", c[0], c[1], c[2], c[3], c[4], c[5]);
}
