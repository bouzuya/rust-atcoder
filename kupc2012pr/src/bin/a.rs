use proconio::input;

fn main() {
    input! {
        m: usize,
        n: usize,
    };

    let ans = match m {
        0 => n + 1,
        1 => 2 + (n + 3) - 3,
        2 => 2 * (n + 3) - 3,
        3 => 2_usize.pow((n + 3) as u32) - 3,
        _ => unreachable!(),
    };
    println!("{}", ans);
}
