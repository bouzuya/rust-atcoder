use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: usize,
    };
    let mut c = std::collections::BTreeMap::new();
    for &s_i in s.iter() {
        *c.entry(s_i).or_insert(0_i64) += 1;
    }
    let x = (*c.get(&'L').unwrap_or(&0) - *c.get(&'R').unwrap_or(&0)).abs();
    let y = (*c.get(&'D').unwrap_or(&0) - *c.get(&'U').unwrap_or(&0)).abs();
    let q = *c.get(&'?').unwrap_or(&0);
    let ans = match t {
        1 => x + y + q,
        2 => {
            if q > x + y {
                (x + y - q).abs() % 2
            } else {
                x + y - q
            }
        }
        _ => unreachable!(),
    };
    println!("{}", ans);
}
