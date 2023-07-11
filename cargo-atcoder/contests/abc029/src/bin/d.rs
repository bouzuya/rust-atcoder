use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let d = {
        let mut m = n;
        let mut d = 0;
        while m > 0 {
            m /= 10;
            d += 1;
        }
        d
    };

    let n = n + 1;
    let mut count = 0;
    let mut l = 10;
    for _ in 0..d {
        let q = n / l;
        let r = n % l;
        let c = q * l / 10
            + if r >= (l / 10 * 2) {
                l / 10
            } else {
                min(r, l / 10 * 2).saturating_sub(l / 10)
            };
        count += c;
        l *= 10;
    }
    let ans = count;
    println!("{}", ans);
}
