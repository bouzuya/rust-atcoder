use proconio::input;

fn main() {
    input! {
        s: String
    };
    let ans = s.starts_with("YAKI");
    println!("{}", if ans { "Yes" } else { "No" });
}
