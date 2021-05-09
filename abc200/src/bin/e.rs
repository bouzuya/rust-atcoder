use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    // 3 つの数字を (a, b, c) とする

    let c2 = |x| if x == 0 { 0 } else { x * (x - 1) / 2 };
    // a + b + c = s のときの (a, b, c) の組み合わせの個数
    let f = |s: usize| -> usize {
        // 二項係数 (nCk) で求める
        // s 個からそれぞれ少なくとも 1 つを選んで 3 つに分ける
        // 少なくとも 1 つを選ぶので -3 する
        // 仕切り 2 つを含めて並び替えるのと同じ
        //
        // 各値が N 以下になっていないといけないことを包除原理で求める
        let c_all = c2(s + 2 - 3);
        let c_1 = c2((s + 2 - 3).saturating_sub(n * 1)) * 3;
        let c_2 = c2((s + 2 - 3).saturating_sub(n * 2)) * 3;
        let c_3 = c2((s + 2 - 3).saturating_sub(n * 3)) * 3;
        c_all + c_2 - c_1 - c_3
    };
    let mut sum = 0_usize;
    // s = a + b + c
    for s in 3..=n * 3 {
        let count = f(s);
        if k <= sum + count {
            // (a, b, c) を順に決める
            // a を決める
            for a in 1..=n {
                let min_b = cmp::max(1, s.saturating_sub(a).saturating_sub(n));
                let max_b = cmp::min(n, s.saturating_sub(a).saturating_sub(1));
                if min_b > max_b {
                    continue;
                }
                let count = max_b - min_b + 1;
                if k <= sum + count {
                    // b を決める
                    for b in min_b..=max_b {
                        let c = s - a - b;
                        if k == sum + 1 {
                            println!("{} {} {}", a, b, c);
                            return;
                        }
                        sum += 1;
                    }
                }
                sum += count;
            }
        }
        sum += count;
    }
}
