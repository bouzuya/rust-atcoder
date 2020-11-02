use proconio::input;

fn main() {
    input! {
        c: char,
    };
    let ans = if ['O', 'P', 'K', 'L'].contains(&c) {
        "Right"
    } else {
        "Left"
    };
    println!("{}", ans);
}
