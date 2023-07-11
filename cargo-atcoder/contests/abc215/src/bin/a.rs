use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ans = s == "Hello,World!";
    println!("{}", if ans { "AC" } else { "WA" });
}
