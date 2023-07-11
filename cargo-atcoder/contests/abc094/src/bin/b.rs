use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        x: usize,
        a: [usize; m],
    };
    let l = a.into_iter().filter(|a_i| a_i < &x).count();
    let r = m - l;
    let ans = l.min(r);
    println!("{}", ans);
}
