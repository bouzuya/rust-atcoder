use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    let mut c = 0_i64;
    let mut s = 0_i64;
    for i in 0.. {
        if s >= x {
            break;
        }
        s += i;
        c = i;
    }
    let ans = c;
    println!("{}", ans);
}
