use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
        y: usize,
    };
    let ans = (0..=std::cmp::max(x * 2, y * 2))
        .filter(|count_c| count_c % 2 == 0)
        .map(|count_c| {
            let count_a = x.saturating_sub(count_c / 2);
            let count_b = y.saturating_sub(count_c / 2);
            a * count_a + b * count_b + c * count_c
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
