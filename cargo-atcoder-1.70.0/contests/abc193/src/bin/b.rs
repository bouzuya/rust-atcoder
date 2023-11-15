use proconio::input;

fn main() {
    input! {
        n: usize,
        apx: [(usize, usize, usize); n],
    }
    let mut min = std::usize::MAX;
    for (a, p, x) in apx {
        let x = x.saturating_sub(a);
        if x > 0 {
            min = min.min(p);
        }
    }
    let ans = if min == std::usize::MAX {
        -1_i64
    } else {
        min as i64
    };
    println!("{}", ans);
}
