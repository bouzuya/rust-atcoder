use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let mut up = 0;
    for i in 1..10 {
        if (a + i) % 10 == b {
            up = i;
        }
    }
    let mut down = 0;
    for i in 1..10 {
        if (10 + a - i) % 10 == b {
            down = i;
        }
    }
    let ans = std::cmp::min(up, down);
    println!("{}", ans);
}
