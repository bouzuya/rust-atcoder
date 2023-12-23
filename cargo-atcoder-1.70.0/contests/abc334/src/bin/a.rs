use proconio::input;

fn main() {
    input! {
        b: usize,
        g: usize,
    };
    let ans = if b > g {
        "Bat"
    } else if b < g {
        "Glove"
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
