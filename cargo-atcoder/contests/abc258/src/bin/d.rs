use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    };
    let mut ans = x * 10_000_000_000_usize;
    let mut c = 0_usize;
    let mut m = 10_000_000_000_usize;
    for i in 0..n {
        let y = i + 1;
        c += ab[i].0 + ab[i].1;
        m = m.min(ab[i].1);
        if y > x {
            break;
        }
        ans = ans.min(c + m * (x - y));
    }
    println!("{}", ans);
}
