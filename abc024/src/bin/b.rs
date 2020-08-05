use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    };
    let mut s = 0_usize;
    for (i, &a_i) in a.iter().enumerate().skip(1) {
        let d = a_i - a[i - 1];
        if d > t {
            s += t;
        } else {
            s += d;
        }
    }
    s += t;
    let ans = s;
    println!("{}", ans);
}
