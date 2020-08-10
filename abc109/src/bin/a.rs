use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let mut found = false;
    for c in 1..=3 {
        if a * b * c % 2 != 0 {
            found = true;
        }
    }
    let ans = found;
    println!("{}", if ans { "Yes" } else { "No" });
}
