use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
    };
    let mut ans = vec![];
    let es = m / 2; // 偶数個ズレた組の部屋の個数 (切り捨て)
    for i in 1..=es {
        ans.push((i, 2 * es + 1 - (i - 1)));
    }
    let os = (m + (2 - 1)) / 2; // 奇数個ズレた組の部屋の個数 (切り上げ)
    for i in 1..=os {
        let b = 2 * es + 1;
        ans.push((b + i, b + 2 * os - (i - 1)));
    }
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}
