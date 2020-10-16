use proconio::input;

fn g1(n: usize, p: usize) -> usize {
    if n == 0 {
        return 0;
    }
    n / p + g1(n / p, p)
}

fn g2(n: usize, p: usize) -> usize {
    (if p == 2 { n / 2 } else { 0 }) + g1(n / 2, p)
}

fn main() {
    input! {
        n: usize,
    };
    if n % 2 != 0 {
        println!("0");
        return;
    }
    let ans = std::cmp::min(g2(n, 2), g2(n, 5));
    println!("{}", ans);
}
