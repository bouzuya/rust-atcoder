use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    };
    let mut ok = 1_000_000_001_usize;
    let mut ng = 0_usize;
    while ok - ng > 1 {
        let mid = ng + (ok - ng) / 2;
        let c_a = a.iter().filter(|&a_i| *a_i <= mid).count();
        let c_b = b.iter().filter(|&b_i| *b_i >= mid).count();
        if c_a >= c_b {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
