use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    };
    let sum_x = xy.iter().map(|(x, _)| x).sum::<usize>();
    let sum_y = xy.iter().map(|(_, y)| y).sum::<usize>();
    let ans = match sum_x.cmp(&sum_y) {
        std::cmp::Ordering::Less => "Aoki",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Takahashi",
    };
    println!("{}", ans);
}
