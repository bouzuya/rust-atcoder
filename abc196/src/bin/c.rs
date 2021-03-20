use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let m = n.to_string().len() / 2 + 1;
    let mut count = 0;
    for i in 1..=10_usize.pow(m as u32) {
        let s: usize = format!("{}{}", i, i).parse().unwrap();
        if s > n {
            continue;
        }
        count += 1;
    }
    let ans = count;
    println!("{}", ans);
}
