use std::cmp;

fn main() {
    for i in 0..100_000_usize {
        let m = 2 * (i + 1);
        let x1 = m * m - 1;
        let x2 = 2 * m * 1;
        let x3 = m * m + 1;
        println!("{} {} {}", cmp::min(x1, x2), cmp::max(x1, x2), x3);
    }
}
