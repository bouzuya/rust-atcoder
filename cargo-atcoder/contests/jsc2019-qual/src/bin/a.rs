use proconio::input;

fn main() {
    input! {
        m: usize,
        d: usize,
    };
    let mut count = 0;
    for x in 1..=m {
        for y in 1..=d {
            let d_10 = y / 10;
            let d_1 = y % 10;
            if d_1 >= 2 && d_10 >= 2 && d_1 * d_10 == x {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
