use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    };
    let mut ok = false;
    let words = vec!["and", "not", "that", "the", "you"];
    for w_i in w {
        if words.iter().any(|word| word == &w_i) {
            ok = true;
        }
    }
    let ans = ok;
    println!("{}", if ans { "Yes" } else { "No" });
}
