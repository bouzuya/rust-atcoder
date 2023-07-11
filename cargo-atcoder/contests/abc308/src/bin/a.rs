use proconio::input;

fn main() {
    input! {
        s: [usize; 8],
    };
    let mut t = s.clone();
    t.sort();
    let ans = s == t
        && s.iter().all(|s_i| (100..=675).contains(s_i))
        && s.iter().all(|s_i| s_i % 25 == 0);
    println!("{}", if ans { "Yes" } else { "No" });
}
