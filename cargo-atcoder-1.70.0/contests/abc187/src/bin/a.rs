use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    };
    fn s(n: usize) -> usize {
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .sum()
    }
    let ans = s(a).max(s(b));
    println!("{}", ans);
}
