use proconio::input;

fn f(x: i64) -> i64 {
    let mut res = 0;
    let mut x = x;
    while x > 0 {
        res += x % 10;
        x /= 10;
    }
    res
}

#[test]
fn name() {
    assert_eq!(1, f(1));
}

fn main() {
    input! {
        n: i64,
    };
    let mut ans = vec![];
    let x_start = std::cmp::max(n - 153, 0);
    for x in x_start..=n {
        if x + f(x) == n {
            ans.push(x);
        }
    }

    println!("{}", ans.len());
    for &ans_i in ans.iter() {
        println!("{}", ans_i);
    }
}
