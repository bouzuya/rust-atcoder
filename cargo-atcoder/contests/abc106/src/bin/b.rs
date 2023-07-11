use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = 0_usize;
    for x in (1..=n).filter(|x| x % 2 != 0) {
        if (1..=x).filter(|d| x % d == 0).count() == 8 {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
