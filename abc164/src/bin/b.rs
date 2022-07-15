use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };

    let count_t = (a + d - 1) / d;
    let count_a = (c + b - 1) / b;
    let ans = count_t >= count_a;
    println!("{}", if ans { "Yes" } else { "No" });
}
