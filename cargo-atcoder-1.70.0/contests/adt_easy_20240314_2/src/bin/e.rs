use proconio::input;

fn f(ans: &mut Vec<char>, n: usize) {
    if n == 0 {
        return;
    }
    if n > 1 && n % 2 == 0 {
        ans.push('B');
        f(ans, n / 2);
    } else {
        ans.push('A');
        f(ans, n - 1);
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut ans = vec![];
    f(&mut ans, n);
    println!("{}", ans.into_iter().rev().collect::<String>());
}
