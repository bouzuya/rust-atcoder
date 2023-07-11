use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let x = n * 108 / 100;
    let ans = if x < 206 {
        "Yay!"
    } else if x == 206 {
        "so-so"
    } else {
        ":("
    };
    println!("{}", ans);
}
