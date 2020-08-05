use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    };
    let mut s = 0_usize;
    for (i, &a_i) in a.iter().enumerate().skip(1) {
        s += std::cmp::min(a_i - a[i - 1], t);
    }
    s += t;
    let ans = s;
    println!("{}", ans);
}
