use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        tx: [(usize, Usize1); n],
    };
    let mut need_max = 0_usize;
    let mut need_sum = 0_usize;
    let mut need = vec![0; n];
    let mut ans = vec![];
    for (t, x) in tx.iter().copied().rev() {
        match t {
            1 => {
                if need[x] > 0 {
                    need[x] -= 1;
                    need_sum -= 1;
                    ans.push(1);
                } else {
                    ans.push(0);
                }
            }
            2 => {
                need[x] += 1;
                need_sum += 1;
                need_max = need_max.max(need_sum);
            }
            _ => unreachable!(),
        }
    }
    if !need.iter().all(|&x| x == 0) {
        println!("-1");
        return;
    }

    ans.reverse();
    println!("{}", need_max);
    for a in ans {
        println!("{}", a);
    }
}
