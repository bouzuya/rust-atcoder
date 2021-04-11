use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = 0;
    for i in 1..=n {
        let j = n - i;
        if i > 0 && j > 0 {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
