use proconio::input;

fn main() {
    input! {
        mut n: [char; 4],
    };
    n.sort();

    let mut chars = "1974".chars().collect::<Vec<char>>();
    chars.sort();

    let ans = n == chars;
    println!("{}", if ans { "YES" } else { "NO" });
}
