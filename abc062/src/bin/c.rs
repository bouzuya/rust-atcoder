use proconio::input;

fn f(h: i64, w: i64) -> i64 {
    let mut res = 1_000_000_000_000;

    // a|b|c
    // a|b|c
    // a|b|c
    for i in 1..w - 1 {
        let j = (w - i) / 2;
        let s_a = i * h;
        let s_b = j * h;
        let s_c = h * w - s_a - s_b;
        let s_max = std::cmp::max(s_a, std::cmp::max(s_b, s_c));
        let s_min = std::cmp::min(s_a, std::cmp::min(s_b, s_c));
        res = std::cmp::min(res, s_max - s_min);
    }

    // a|bb
    // a+--
    // a|cc
    for i in 1..w {
        let j = h / 2;
        let s_a = i * h;
        let s_b = j * (w - i);
        let s_c = h * w - s_a - s_b;
        let s_max = std::cmp::max(s_a, std::cmp::max(s_b, s_c));
        let s_min = std::cmp::min(s_a, std::cmp::min(s_b, s_c));
        res = std::cmp::min(res, s_max - s_min);
    }

    res
}

fn main() {
    input! {
        h: i64,
        w: i64
    };
    let ans = std::cmp::min(f(h, w), f(w, h));
    println!("{}", ans);
}
