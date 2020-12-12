use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut count = vec![0; n];
    for a_i in a {
        count[a_i] += 1;
    }
    match (
        count.iter().position(|&c| c == 0),
        count.iter().position(|&c| c == 2),
    ) {
        (None, None) => println!("Correct"),
        (Some(_), None) | (None, Some(_)) => unreachable!(),
        (Some(s), Some(d)) => println!("{} {}", d + 1, s + 1),
    }
}
