use proconio::input;

fn main() {
    input! {
        h: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };

    let mut min = 1 << 60;
    for i in 0..100 {
        let mut price = 0_usize;
        let mut h = h;
        for _ in 0..i {
            price += d;
            h = h.saturating_sub(c);
            if h == 0 {
                break;
            }
            h -= h / 2;
        }
        if h > 0 {
            price += (h + a - 1) / a * b;
        }
        min = min.min(price);
    }
    let ans = min;
    println!("{}", ans);
}
