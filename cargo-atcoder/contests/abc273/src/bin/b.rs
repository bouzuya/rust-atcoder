use proconio::input;

fn main() {
    input! {
        mut x: usize,
        k: usize,
    };
    for i in 0..k {
        if x == 0 {
            break;
        }
        let b = 10_usize.pow((i + 1) as u32);
        let d = x % b / (b / 10);
        x = if d >= 5 { x / b + 1 } else { x / b } * b;
    }
    let ans = x;
    println!("{}", ans);
}
