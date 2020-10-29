use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(i64, i64); n],
    };
    let (mut l, mut r) = ta[0];
    for (t_i, a_i) in ta.into_iter().skip(1) {
        if t_i == a_i {
            l = std::cmp::max(l, r);
            r = l;
        } else {
            let l1 = l + if l % t_i == 0 { 0 } else { t_i - l % t_i };
            let r1 = l1 / t_i * a_i;
            let r2 = r + if r % a_i == 0 { 0 } else { a_i - r % a_i };
            let l2 = r2 / a_i * t_i;
            l = if l1 < l || r1 < r { l2 } else { l1 };
            r = if l1 < l || r1 < r { r2 } else { r1 };
        }
    }

    let ans = l + r;
    println!("{}", ans);
}
