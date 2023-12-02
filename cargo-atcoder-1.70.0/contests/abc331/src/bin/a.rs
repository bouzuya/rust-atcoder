use proconio::input;

fn main() {
    input! {
        capital_m: usize,
        capital_d: usize,
        y: usize,
        m: usize,
        d: usize,
    };

    let nd = if d == capital_d { 1 } else { d + 1 };
    let nm = if d == capital_d {
        if m == capital_m {
            1
        } else {
            m + 1
        }
    } else {
        m
    };
    let ny = if d == capital_d && m == capital_m {
        y + 1
    } else {
        y
    };
    println!("{} {} {}", ny, nm, nd);
}
