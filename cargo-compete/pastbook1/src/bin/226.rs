use proconio::input;

fn main() {
    input! {
        a: usize,
        r: usize,
        n: usize,
    }

    let ans = r
        .checked_pow(n as u32 - 1)
        .and_then(|p| a.checked_mul(p))
        .and_then(|x| if x > 1_000_000_000 { None } else { Some(x) })
        .map(|x| x.to_string())
        .unwrap_or("large".to_string());
    println!("{}", ans);
}
