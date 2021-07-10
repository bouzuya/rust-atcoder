use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };
    let mut sum = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        let p = if i % 2 != 0 { a_i - 1 } else { a_i };
        sum += p;
    }
    let ans = sum <= x;
    println!("{}", if ans { "Yes" } else { "No" });
}
