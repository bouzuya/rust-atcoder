use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
    };
    let count = a
        .iter()
        .zip(a.iter().rev())
        .take(a.len() / 2)
        .filter(|(&a_i, &a_j)| a_i != a_j)
        .count();
    let mut sum = if a.len() % 2 != 0 && count != 0 {
        25
    } else {
        0
    };
    for (&a_i, &a_j) in a.iter().zip(a.iter().rev()).take(a.len() / 2) {
        match (count == 0, a_i == a_j) {
            (true, true) => sum += 25 * 2,
            (true, false) => unreachable!(),
            (false, true) => sum += 25 * 2,
            (false, false) => sum += if count == 1 { 24 } else { 25 } * 2,
        }
    }
    let ans = sum;
    println!("{}", ans);
}
