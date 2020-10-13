use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let w = vec!["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let ans = w.len() - w.into_iter().position(|w_i| w_i == s).unwrap();
    println!("{}", ans);
}
