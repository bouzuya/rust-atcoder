use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    let &max = a.iter().max().unwrap();
    let mut r = max;
    let mut d = 1_000_000_001;
    for &a_i in a.iter() {
        if a_i == max {
            continue;
        }
        let d_i = (a_i - max / 2).abs();
        if d_i < d {
            r = a_i;
            d = d_i;
        }
    }
    println!("{} {}", max, r);
}
