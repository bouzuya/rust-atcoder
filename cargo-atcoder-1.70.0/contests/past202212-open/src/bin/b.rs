use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };

    let ad = a * d;
    let bc = b * c;
    let ans = match (b > 0, d > 0) {
        (true, true) | (false, false) => ad.cmp(&bc),
        (true, false) | (false, true) => bc.cmp(&ad),
    };
    let ans = match ans {
        std::cmp::Ordering::Less => '<',
        std::cmp::Ordering::Equal => '=',
        std::cmp::Ordering::Greater => '>',
    };
    println!("{}", ans);
}
