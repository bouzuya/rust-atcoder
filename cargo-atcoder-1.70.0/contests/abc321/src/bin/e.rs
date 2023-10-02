use proconio::input;

/// x から k 下がった深さの個数を求める
fn g(n: usize, x: usize, k: usize) -> usize {
    if x > n {
        return 0;
    }

    let (mut l, mut r) = (x, x);
    for _ in 0..k {
        l *= 2;
        r = r * 2 + 1;
        if l > n {
            return 0;
        }
    }
    r = r.min(n);

    r + 1 - l
}

fn f(n: usize, mut x: usize, mut k: usize) -> usize {
    // 下がるだけ
    let mut sum = g(n, x, k);
    // 1つ上がってから下がる
    while x > 1 && k >= 2 {
        sum += g(n, x ^ 1, k - 2);
        k -= 1;
        x /= 2;
    }
    // 1つ上がるだけ
    if x != 1 && k == 1 {
        sum += 1;
    }
    sum
}

fn main() {
    input! {
        t: usize,
        nxk: [(usize, usize, usize); t],
    }
    for (n, x, k) in nxk {
        let ans = f(n, x, k);
        println!("{}", ans);
    }
}
