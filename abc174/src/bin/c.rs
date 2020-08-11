use proconio::input;

fn main() {
    input! {
        k: i64,
    };
    let mut found = None;
    let mut x = 7;
    for i in 1..=k {
        if x % k == 0 {
            found = Some(i);
            break;
        }
        x = (x * 10 + 7) % k;
    }
    let ans = found.unwrap_or(-1);
    println!("{}", ans);
}
