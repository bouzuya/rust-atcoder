use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut ans = vec![];
    while n > 0 {
        if n % 2 == 0 {
            ans.push('B');
            n /= 2;
        } else {
            ans.push('A');
            n -= 1;
        }
    }

    ans.reverse();
    let ans = ans.into_iter().collect::<String>();
    println!("{}", ans);
}
