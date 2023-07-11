use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
        d: usize,
    };
    let mut count = 0;
    for x in l..=r {
        if x % d == 0 {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
