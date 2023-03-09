use proconio::input;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    };

    let mut count = 0;
    while n != 0 {
        n /= k;
        count += 1;
    }

    let ans = count;
    println!("{}", ans);
}
