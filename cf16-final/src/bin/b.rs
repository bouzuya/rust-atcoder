use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    for x in 1..=n {
        let is_ok = x == n || x * (x + 1) >= 2 * n;
        if is_ok {
            let mut a = vec![x];
            let mut sum = x;
            for i in (1..x).rev() {
                if sum + i <= n {
                    a.push(i);
                    sum += i;
                }
            }
            for &a_i in a.iter().rev() {
                println!("{}", a_i);
            }
            break;
        }
    }
}
