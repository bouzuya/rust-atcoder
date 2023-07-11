use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(usize, usize); n],
    };
    let mut c = ta[0];
    for (t_i, a_i) in ta.into_iter().skip(1) {
        let x = ((c.0 + t_i - 1) / t_i).max((c.1 + a_i - 1) / a_i);
        c = (t_i * x, a_i * x);
    }
    let ans = c.0 + c.1;
    println!("{}", ans);
}
