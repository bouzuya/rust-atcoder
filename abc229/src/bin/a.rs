use proconio::input;

fn main() {
    input! {
        s: [String; 2],
    };

    let ans = !((s[0] == "#." && s[1] == ".#") || (s[0] == ".#" && s[1] == "#."));
    println!("{}", if ans { "Yes" } else { "No" });
}
