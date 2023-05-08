use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    }

    ab.sort_by_key(|(a, b)| a + b);
    ab.reverse();
    let mut f = 0_i64;
    let mut s = 0_i64;
    for (i, (a, b)) in ab.iter().copied().enumerate() {
        if i % 2 == 0 {
            f += a;
        } else {
            s += b;
        }
    }
    let ans = f - s;
    println!("{}", ans);
}
