use proconio::input;

fn main() {
    input! {
        mut n: i64,
    };

    let mut ans = vec![];
    let mut sum = 0_i64;
    for x in 0.. {
        if n - sum == 0 {
            break;
        }
        let r = (n - sum) % 2_i64.pow(x + 1);
        if r != 0 {
            ans.push(1);
            sum += (-2_i64).pow(x);
        } else {
            ans.push(0);
        }
    }
    if ans.is_empty() {
        ans.push(0);
    }
    ans.reverse();
    println!(
        "{}",
        ans.into_iter()
            .map(|i| (b'0' + i as u8) as char)
            .collect::<String>()
    );
}
