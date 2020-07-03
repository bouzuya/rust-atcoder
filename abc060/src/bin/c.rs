use proconio::input;

fn main() {
    input! {
        n: usize,
        t_l: i64,
        t: [i64; n],
    };
    let mut s = 0;
    let mut l = 0;
    let mut r = t_l;
    for &t_i in t.iter() {
        if t_i <= r {
            r = t_i + t_l;
        } else {
            s += r - l;
            l = t_i;
            r = t_i + t_l;
        }
    }
    s += r - l;
    let ans = s;
    println!("{}", ans);
}
