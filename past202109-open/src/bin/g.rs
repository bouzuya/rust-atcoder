use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        abc: [(usize, usize, usize); n],
    };
    let mut ng = 0_usize;
    let mut ok = std::usize::MAX;
    while ok - ng > 1 {
        let x = ng + (ok - ng) / 2;
        let mut count = 0;
        for (a, b, c) in abc.iter().copied() {
            count += (if x < b { 0 } else { (x - b) / c + 1 }).min(a);
        }
        if count >= k {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
