use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        t: [i64; 2],
        a: [i64; 2],
        b: [i64; 2],
    };
    let p = (a[0] - b[0]) * t[0];
    let q = (a[1] - b[1]) * t[1];
    let (p, q) = if p > 0 { (-p, -q) } else { (p, q) };
    match (p + q).cmp(&0) {
        Ordering::Less => println!("0"),
        Ordering::Equal => println!("infinity"),
        Ordering::Greater => {
            let s = -p / (p + q);
            let t = -p % (p + q);
            let ans = if t != 0 { s * 2 + 1 } else { s * 2 };
            println!("{}", ans);
        }
    }
}
