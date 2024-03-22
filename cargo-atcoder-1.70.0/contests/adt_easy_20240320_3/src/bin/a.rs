use proconio::input;

fn main() {
    input! {
        capital_m: usize,
        capital_d: usize,
        y: usize,
        m: usize,
        d: usize,
    };
    if d != capital_d {
        println!("{} {} {}", y, m, d + 1);
    } else if m != capital_m {
        println!("{} {} {}", y, m + 1, 1);
    } else {
        println!("{} {} {}", y + 1, 1, 1);
    }
}
