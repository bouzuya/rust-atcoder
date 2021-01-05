use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut n: Chars,
    };
    n.reverse();

    let mut o = 0_usize;
    let mut e = 0_usize;
    for i in (0..n.len()).step_by(2) {
        o += (n[i] as u8 - '0' as u8) as usize;
        if i + 1 < n.len() {
            e += (n[i + 1] as u8 - '0' as u8) as usize;
        }
    }

    println!("{} {}", e, o);
}
