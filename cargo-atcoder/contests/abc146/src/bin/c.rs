use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    };
    let d = |n: usize| n.to_string().len();
    let mut ok = 0;
    let mut ng = 1_000_000_001;
    while ng - ok > 1 {
        let n = ok + (ng - ok) / 2;
        if a * n + b * d(n) <= x {
            ok = n;
        } else {
            ng = n;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
