use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let b = a.iter().map(|a_i| a_i.abs()).collect::<Vec<i64>>();
    let ans = b.iter().sum::<i64>()
        - if a.iter().copied().filter(|&a_i| a_i < 0).count() % 2 == 0 {
            0
        } else {
            *b.iter().min().unwrap() * 2
        };
    println!("{}", ans);
}
