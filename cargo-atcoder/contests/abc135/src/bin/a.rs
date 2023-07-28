use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = if (a + b) % 2 == 0 {
        format!("{}", (a + b) / 2)
    } else {
        "IMPOSSIBLE".to_owned()
    };
    println!("{}", ans);
}
