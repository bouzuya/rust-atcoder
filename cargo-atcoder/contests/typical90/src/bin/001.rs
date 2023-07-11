use proconio::input;

fn f(_n: usize, l: i64, k: usize, a: &Vec<i64>, x: i64) -> bool {
    let mut count = 0;
    let mut p = 0;
    for &a_i in a.iter() {
        if a_i - p >= x {
            p = a_i;
            count += 1;
        }
    }
    if l - p >= x {
        count += 1;
    }
    count >= k + 1
}

fn main() {
    input! {
        n: usize,
        l: i64,
        k: usize,
        a: [i64; n],
    };
    let mut ok = 0_i64;
    let mut ng = 1_000_000_001_i64;
    while ng - ok > 1 {
        let x = ok + (ng - ok) / 2;
        if f(n, l, k, &a, x) {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
