use proconio::input;

fn main() {
    input! {
        n: usize,
        av: [usize; n]
    };
    let ans = if av
        .iter()
        .filter(|&a| a % 2 == 0)
        .all(|a| a % 3 == 0 || a % 5 == 0)
    {
        "APPROVED"
    } else {
        "DENIED"
    };
    println!("{}", ans);
}
