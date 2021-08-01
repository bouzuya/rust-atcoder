use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = match (a > 0, b > 0) {
        (false, false) => unreachable!(),
        (false, true) => "Silver",
        (true, false) => "Gold",
        (true, true) => "Alloy",
    };
    println!("{}", ans);
}
