use proconio::input;

fn g(_: usize, b: usize, c: usize) -> Option<usize> {
    if b % 3 != c % 3 {
        return None;
    }
    Some((b - c) / 3 * 3 + c)
}

fn f(a: usize, b: usize, c: usize) -> Option<usize> {
    vec![
        g(a, b.max(c), b.min(c)),
        g(b, a.max(c), a.min(c)),
        g(c, a.max(b), a.min(b)),
    ]
    .into_iter()
    .filter_map(|x| x)
    .min()
}

fn main() {
    input! {
        t: usize,
        rgb: [(usize, usize, usize); t],
    };
    for (r, g, b) in rgb {
        println!("{}", f(r, g, b).map(|x| x as i64).unwrap_or(-1));
    }
}
