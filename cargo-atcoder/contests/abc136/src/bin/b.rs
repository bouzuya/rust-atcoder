use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = 0_usize;
    for x in 1..=n {
        if x.to_string().len() % 2 != 0 {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
