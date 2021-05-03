use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let mut count = 0;
    for x in 1..=12 {
        if x > a || (x == a && x > b) {
            break;
        }
        count = x;
    }
    let ans = count;
    println!("{}", ans);
}
