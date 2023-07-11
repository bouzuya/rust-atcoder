use proconio::input;

fn main() {
    input! {
        s: char,
        t: char,
    };
    let ans = if s == 'Y' {
        t.to_uppercase().to_string()
    } else {
        t.to_string()
    };
    println!("{}", ans);
}
