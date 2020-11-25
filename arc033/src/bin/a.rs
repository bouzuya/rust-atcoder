use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = 0;
    for i in 0..n {
        for _ in i..n {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
