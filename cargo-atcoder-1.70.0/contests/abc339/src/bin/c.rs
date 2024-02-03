use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut min = 0_i64;
    let mut cur = 0_i64;
    for a_i in a.iter().copied() {
        cur += a_i;
        min = min.min(cur);
    }
    let ans = -min + cur;
    println!("{}", ans);
}
