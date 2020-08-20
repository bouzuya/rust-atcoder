use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };
    let abc = ab
        .iter()
        .map(|&(a_i, b_i)| (a_i, b_i, a_i - b_i))
        .collect::<Vec<_>>();
    let mut ab1 = abc
        .iter()
        .filter(|&(_, _, c_i)| *c_i < 0)
        .collect::<Vec<_>>();
    ab1.sort_by_key(|&(a_i, _, _)| a_i);
    let ab2 = abc
        .iter()
        .filter(|&(_, _, c_i)| *c_i == 0)
        .collect::<Vec<_>>();
    let mut ab3 = abc
        .iter()
        .filter(|&(_, _, c_i)| *c_i > 0)
        .collect::<Vec<_>>();
    ab3.sort_by_key(|&(_, b_i, _)| -b_i);
    let mut ans = 0;
    let mut v = 0;
    for (a_i, b_i, _) in ab1.iter() {
        v += a_i;
        ans = std::cmp::max(ans, v);
        v -= b_i;
    }
    for (a_i, b_i, _) in ab2.iter() {
        v += a_i;
        ans = std::cmp::max(ans, v);
        v -= b_i;
    }
    for (a_i, b_i, _) in ab3.iter() {
        v += a_i;
        ans = std::cmp::max(ans, v);
        v -= b_i;
    }
    println!("{}", ans);
}
