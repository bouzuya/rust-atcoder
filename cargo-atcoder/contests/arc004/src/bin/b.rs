use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [i64; n],
    };
    let mut max = 0;
    let mut max_d = 0;
    for &d_i in d.iter() {
        max += d_i;
        max_d = std::cmp::max(max_d, d_i);
    }
    let min = std::cmp::max(0, max_d - (max - max_d));
    println!("{}\n{}", max, min);
}
