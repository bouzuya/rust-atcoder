use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut min = 1_000_000_000_000;
    for a in 1..n {
        let b = n - a;
        let x = a
            .to_string()
            .chars()
            .map(|c| (c as u8 - b'0') as usize)
            .sum::<usize>()
            + b.to_string()
                .chars()
                .map(|c| (c as u8 - b'0') as usize)
                .sum::<usize>();
        min = min.min(x);
    }
    let ans = min;
    println!("{}", ans);
}
