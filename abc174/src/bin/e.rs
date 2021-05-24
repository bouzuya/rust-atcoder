use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut ok = 1_000_000_000_usize;
    let mut ng = 0;
    while ok - ng > 1 {
        let m = (ok + ng) / 2;
        let mut count = 0;
        for &a_i in a.iter() {
            count += (a_i + m - 1) / m - 1;
        }
        if count <= k {
            ok = m;
        } else {
            ng = m;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
