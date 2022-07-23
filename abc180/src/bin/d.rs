use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
    };
    let mut ans = 0;
    let mut cur = x;
    for i in 1.. {
        if cur.checked_mul(a).is_some() && cur * a <= cur + b && cur * a < y {
            ans = i;
            cur *= a;
        } else {
            let ans = match (cur * a >= y, cur + b >= y) {
                (true, true) => i - 1,
                (false, true) => unreachable!(),
                (_, false) => i - 1 + (y - 1 - cur) / b,
            };
            println!("{}", ans);
            return;
        }
    }
    println!("{}", ans);
}
