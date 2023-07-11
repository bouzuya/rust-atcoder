use proconio::input;

fn f(lt: bool, r: &[i64]) -> usize {
    let mut is_lt = lt;
    let mut c_s = 0;
    let mut r_p = r[0];
    for &r_i in r.iter().skip(1) {
        if is_lt {
            if r_p < r_i {
                is_lt = !is_lt;
            } else {
                c_s += 1;
            }
        } else {
            if r_p > r_i {
                is_lt = !is_lt;
            } else {
                c_s += 1;
            }
        }
        r_p = r_i;
    }
    c_s
}

fn main() {
    input! {
        n: usize,
        r: [i64; n],
    };
    if n < 3 {
        println!("{}", 0);
        return;
    }
    let lt = f(true, &r);
    let gt = f(false, &r);
    let ans = r.len() - std::cmp::min(lt, gt);
    let ans = if ans < 3 { 0 } else { ans };
    println!("{}", ans);
}
