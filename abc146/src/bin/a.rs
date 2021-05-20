use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let w = vec!["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let i = w.iter().position(|w_i| w_i == &s).unwrap();
    let ans = w.len() - i;
    println!("{}", ans);
}
