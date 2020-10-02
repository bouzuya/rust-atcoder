use proconio::input;

fn main() {
    input! {
        s: String
    };
    println!("{}{}", s, if s.ends_with("s") { "es" } else { "s" });
}
