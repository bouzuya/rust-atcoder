use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };
    let mut c = vec![];
    for (i, &(a_i, b_i)) in ab.iter().enumerate() {
        c.push((a_i + b_i, i));
    }
    c.sort_by_key(|(k, _)| -k);

    let mut odd = true;
    let mut s_a = 0_i64;
    let mut s_b = 0_i64;
    for &(_, i) in c.iter() {
        if odd {
            s_a += ab[i].0;
        } else {
            s_b += ab[i].1;
        }
        odd = !odd;
    }
    let ans = s_a - s_b;
    println!("{}", ans);
}
