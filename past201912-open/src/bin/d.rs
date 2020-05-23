use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut c = vec![0; n];
    for &a_i in a.iter() {
        c[a_i] += 1;
    }
    match (
        c.iter().position(|&c_i| c_i == 2),
        c.iter().position(|&c_i| c_i == 0),
    ) {
        (None, None) => println!("Correct"),
        (Some(y), Some(x)) => println!("{} {}", y + 1, x + 1),
        _ => unreachable!(),
    }
}
