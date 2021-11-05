use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c: Chars,
    };
    let mut max = 0;
    let mut min = c.len();
    for x in 1..=4 {
        let mut count = 0;
        for c_i in c.iter().copied() {
            if c_i as u8 - b'0' == x {
                count += 1;
            }
        }
        max = max.max(count);
        min = min.min(count);
    }
    println!("{} {}", max, min);
}
