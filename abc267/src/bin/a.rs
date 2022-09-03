use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let mut weekdays = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    weekdays.reverse();
    let index = weekdays.iter().position(|w| w == &s).unwrap();
    let ans = index + 1;
    println!("{}", ans);
}
