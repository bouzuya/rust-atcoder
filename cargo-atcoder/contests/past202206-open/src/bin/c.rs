use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut ans = vec![];
    for k in 1..=m {
        let c = match n.checked_pow(k as u32) {
            Some(nk) => {
                if nk <= 1_000_000_000_usize {
                    'o'
                } else {
                    'x'
                }
            }
            None => 'x',
        };
        ans.push(c);
    }
    println!("{}", ans.iter().collect::<String>());
}
