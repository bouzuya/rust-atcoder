use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let iv: Vec<usize> = s
        .iter()
        .map(|&c| match c {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        })
        .collect();
    let mut cv = vec![0_usize; 3];
    for &i in iv.iter() {
        cv[i] += 1;
    }
    // (全体) R の個数 * G の個数 * B の個数
    let u = cv.iter().fold(1_usize, |acc, &c| acc * c);
    // 全体から部分 j - i = k - j を取り除く (j - i != k - j を求める)
    let mut a = 0;
    for i in 0..n {
        for j in i + 1..n {
            let k = 2 * j - i;
            if k < n && (iv[i] != iv[j]) && (iv[i] != iv[k]) && (iv[j] != iv[k]) {
                a += 1;
            }
        }
    }
    let ans = u - a;
    println!("{}", ans);
}
