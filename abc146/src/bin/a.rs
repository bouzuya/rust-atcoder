use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let wdays = vec!["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let ans = 7 - wdays.iter().position(|wday| &s == wday).unwrap();
    println!("{}", ans);
}
