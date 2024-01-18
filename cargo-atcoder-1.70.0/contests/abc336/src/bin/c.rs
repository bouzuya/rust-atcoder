use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };

    if n == 1 {
        println!("0");
        return;
    }

    n -= 1;
    let d = "02468".chars().collect::<Vec<char>>();
    let mut ans = vec![];
    while n != 0 {
        ans.push(d[n % 5]);
        n /= 5;
    }
    println!("{}", ans.into_iter().rev().collect::<String>());
}
