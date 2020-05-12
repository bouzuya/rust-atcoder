use proconio::input;
use proconio::marker::Chars;

fn f(p: &mut Vec<(i64, i64)>) -> bool {
    p.sort_by_key(|&p_i| std::cmp::Reverse(p_i));
    let mut c = 0;
    for &(b, r) in p.iter() {
        if c + b < 0 {
            return false;
        }
        c += r;
    }
    true
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut sum = 0;
    let mut p_l = vec![];
    let mut p_r = vec![];
    for s_i in s.iter() {
        // 初期位置から b まで下がって r で終わるものという表現に変換する
        let (b, r) = s_i.iter().fold((0, 0), |(b, r), &c| match c {
            '(' => (b, r + 1),
            ')' => (std::cmp::min(b, r - 1), r - 1),
            _ => unreachable!(),
        });
        // 左半分に配置するものと右半分に配置するものに分ける
        // 右半分の配置は左右を反転させたものとして解釈する
        if r > 0 {
            p_l.push((b, r));
        } else {
            p_r.push((b - r, -r))
        }
        sum += r;
    }
    let ans = if sum == 0 && f(&mut p_l) && f(&mut p_r) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
