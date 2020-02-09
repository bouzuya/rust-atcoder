use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        mut n: Bytes,
        k: usize
    };
    for i in 0..n.len() {
        n[i] -= b'0';
    }
    let ans = match k {
        1 => 9 * (n.len() - 1) + n[0] as usize,
        2 => {
            let c = 0;
            (n.len() * 2) * 81 + (n.len() * 9) * (n[0] as usize - 1)
        }
        _ => 0,
    };
    println!("{}", ans);
}
