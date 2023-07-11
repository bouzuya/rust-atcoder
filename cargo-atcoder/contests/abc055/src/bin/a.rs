use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut x = 0;
    let mut y = 0;
    for i in 1..=n {
        x += 800;
        if i % 15 == 0 {
            y += 200;
        }
    }
    let ans = x - y;
    println!("{}", ans);
}
