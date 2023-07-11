use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        mut ab: [(usize, usize); m],
    };
    ab.sort();
    let mut count = 1;
    let mut l = ab[0].0;
    let mut r = ab[0].1;
    for (l_i, r_i) in ab.into_iter().skip(1) {
        if (l..r).contains(&l_i) {
            r = r.min(r_i);
        } else {
            count += 1;
            l = l_i;
            r = r_i;
        }
    }
    let ans = count;
    println!("{}", ans);
}
