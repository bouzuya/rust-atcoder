use proconio::input;

fn main() {
    input! {
        mut s: [String; 15]
    };
    s.sort();
    let ans = &s[7 - 1];
    println!("{}", ans);
}
