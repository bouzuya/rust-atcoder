use proconio::input;

fn main() {
    let n = 3;
    input! {
        abc: [usize; n]
    };
    let set: std::collections::HashSet<usize> = abc.into_iter().collect();
    let ans = if set.len() == n - 1 { "Yes" } else { "No" };
    println!("{}", ans);
}
