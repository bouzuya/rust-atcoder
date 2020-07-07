use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        ct: [(usize, usize); n],
    };
    let inf = 1_000_000;
    let mut ans = inf;
    for &(c_i, t_i) in ct.iter() {
        if t_i <= t {
            ans = std::cmp::min(ans, c_i);
        }
    }
    if ans == inf {
        println!("TLE");
    } else {
        println!("{}", ans);
    }
}
