use proconio::input;

fn main() {
    input! {
        l: usize
    };
    let x = l as f64 / 3.0f64;
    let mut ans: f64 = x * x * x;
    for a in 1..=l {
        for b in 1..=a {
            if l < a + b {
                continue;
            }
            let c = l - a - b;
            let x = (a * b * c) as f64;
            ans = if ans < x { x } else { ans };
        }
    }
    println!("{}", ans);
}
