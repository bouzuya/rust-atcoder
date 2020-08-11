use proconio::input;

fn is_ok(a: &[i64], k: i64, x: i64) -> bool {
    let mut c = 0;
    for &a_i in a.iter() {
        c += (a_i + (x - 1)) / x - 1;
    }
    c <= k
}

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    };
    let mut ng = 0;
    let mut ok = 1_000_000_001;
    while ok - ng > 1 {
        let x = ng + (ok - ng) / 2;
        if is_ok(&a, k, x) {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
