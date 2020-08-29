// WA
use proconio::input;
use proconio::marker::Usize1;

fn is_ok(w: usize, ab: &[(usize, usize)], x: usize, ans: &mut Vec<usize>) -> bool {
    let mut updated = false;
    let mut count = 0_usize;
    let mut x = x;
    for (i, &(a_i, b_i)) in ab.iter().enumerate() {
        count += 1;
        let d = if x < a_i {
            0
        } else if b_i < w {
            b_i - x
        } else {
            break;
        };
        x += d as usize;
        count += d;
        if count < ans[i] {
            ans[i] = count;
            updated = true;
        }
    }
    updated
}

fn main() {
    input! {
        h: usize,
        w: usize,
        ab: [(Usize1, usize); h],
    };
    let inf = 1_000_000_000_usize;
    let mut ans = vec![inf; h];

    for &ans_i in ans.iter() {
        println!("{}", if ans_i == inf { -1 } else { ans_i as i64 });
    }
}
