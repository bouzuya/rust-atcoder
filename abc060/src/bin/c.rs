use proconio::input;

fn main() {
    input! {
        n: usize,
        t_l: i64,
        t: [i64; n],
    };
    let mut s = 0;
    for (t_i0, t_i1) in t.windows(2).map(|w| match w {
        &[t_i0, t_i1] => (t_i0, t_i1),
        _ => unreachable!(),
    }) {
        s += std::cmp::min(t_l, t_i1 - t_i0);
    }
    s += t_l;
    let ans = s;
    println!("{}", ans);
}
