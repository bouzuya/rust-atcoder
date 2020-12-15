use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    };
    if m >= n {
        println!("0");
        return;
    }
    a.sort();
    a.push(n + 1);
    let mut counts = vec![];
    let mut p = 0;
    for &a_i in a.iter() {
        let c = a_i - p - 1;
        if c > 0 {
            counts.push(c);
        }
        p = a_i;
    }
    if counts.is_empty() {
        println!("1");
        return;
    }
    let k = *counts.iter().min().unwrap();
    let ans = counts.iter().map(|c| (c + k - 1) / k).sum::<usize>();
    println!("{}", ans);
}
