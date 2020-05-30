use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let mut tbl = vec![0_usize; n + 1];
    for i in 0..=n {
        if i == 0 {
            tbl[i] = 2;
        } else if i == 1 {
            tbl[i] = 1;
        } else if i >= 2 {
            tbl[i] = tbl[i - 1] + tbl[i - 2];
        } else {
            unreachable!()
        }
    }
    let ans = tbl[n];
    println!("{}", ans);
}
