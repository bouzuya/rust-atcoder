use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        mut ab: [(i64, i64); m],
    };
    ab.sort_by_key(|&(_, b)| b);
    let mut c = 0;
    let mut l = 0;
    for &(a_i, b_i) in ab.iter() {
        if a_i >= l {
            c += 1;
            l = b_i;
        }
    }
    let ans = c;
    println!("{}", ans);
}
