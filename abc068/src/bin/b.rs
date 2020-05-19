use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let mut ans = 0;
    for i in 0.. {
        let x = 2_usize.pow(i);
        if x <= n {
            ans = x;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
