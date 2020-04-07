use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let mut ans = n;
    for x in 1..=n {
        let y = n / x;
        if x * y > n {
            break;
        }
        ans = std::cmp::min(ans, n - x * y + if x > y { x - y } else { y - x });
    }
    println!("{}", ans);
}
