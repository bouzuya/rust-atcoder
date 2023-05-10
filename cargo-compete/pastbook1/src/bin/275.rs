use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    }

    let mut ok = 0_usize;
    let mut ng = x;
    while ng - ok > 1 {
        let n = ok + (ng - ok) / 2;
        let d_n = n.to_string().len();
        if a * n + b * d_n <= x && n <= 1_000_000_000 {
            ok = n;
        } else {
            ng = n;
        }
    }
    println!("{}", ok);
}
