use proconio::input;

fn main() {
    input! {
        s: String,
    };
    if s.ends_with("er") {
        println!("er");
    } else if s.ends_with("ist") {
        println!("ist");
    }
}
