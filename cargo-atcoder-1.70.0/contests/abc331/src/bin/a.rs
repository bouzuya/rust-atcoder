use proconio::input;

fn main() {
    input! {
        capital_m: usize,
        capital_d: usize,
        y: usize,
        m: usize,
        d: usize,
    };

    let mut ans = (y, m, d + 1);
    if ans.2 > capital_d {
        ans = (y, m + 1, 1);
        if ans.1 > capital_m {
            ans = (y + 1, 1, 1);
        }
    }
    println!("{} {} {}", ans.0, ans.1, ans.2);
}
