use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    };
    let mut ok = 0;
    let mut ng = 1_000_000_001;
    while ng - ok > 1 {
        let n = ok + (ng - ok) / 2;
        let d = n.to_string().len();
        if a * n + b * d <= x {
            ok = n;
        } else {
            ng = n;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
