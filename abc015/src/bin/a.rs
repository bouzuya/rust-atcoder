use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    };
    let ans = if a.len() == b.len() {
        unreachable!()
    } else if a.len() > b.len() {
        a
    } else {
        b
    };
    println!("{}", ans);
}
