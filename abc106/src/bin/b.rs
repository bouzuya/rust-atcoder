use proconio::input;

fn main() {
    input! { n: usize };
    let mut ans = 0;
    for i in 1..=n {
        if i % 2 == 0 {
            continue;
        }
        let mut count = 0;
        for j in 1..=i {
            if i % j == 0 {
                count += 1;
                if count > 8 {
                    break;
                }
            }
        }
        if count == 8 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
