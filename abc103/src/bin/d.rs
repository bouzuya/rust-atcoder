use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        mut ab: [(i64, i64); m],
    };
    ab.sort_by_key(|&(_, b)| b);
    let mut ans = 1;
    let mut c = ab[0].1;
    for &(a_i, b_i) in ab.iter() {
        if a_i < c {
            continue;
        }
        c = b_i;
        ans += 1;
    }
    println!("{}", ans);
}
