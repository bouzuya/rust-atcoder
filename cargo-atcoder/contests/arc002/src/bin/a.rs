use proconio::input;

fn is_leap_year(y: i64) -> bool {
    if y % 400 == 0 {
        return true;
    }
    if y % 100 == 0 {
        return false;
    }
    if y % 4 == 0 {
        return true;
    }
    return false;
}

fn main() {
    input! {
        y: i64,
    };
    let ans = is_leap_year(y);
    println!("{}", if ans { "YES" } else { "NO" });
}
