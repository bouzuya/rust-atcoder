use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    };

    let g = |x: usize, a: usize, b: usize, c: usize| {
        let q = a + c;
        let p = x / q;
        let r = x % q;
        (p * a + r.min(a)) * b
    };

    let ans = match g(x, a, b, c).cmp(&g(x, d, e, f)) {
        std::cmp::Ordering::Less => "Aoki",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Takahashi",
    };
    println!("{}", ans);
}
