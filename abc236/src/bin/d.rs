use proconio::input;

fn dfs(
    n: usize,
    a: &[Vec<usize>],
    ans: &mut usize,
    pairs: &mut Vec<(usize, usize)>,
    used: &mut Vec<bool>,
    index: usize,
) {
    if index == 2 * n {
        let mut b = 0_usize;
        for (i, j) in pairs.iter().copied() {
            b ^= a[i][j];
        }
        *ans = (*ans).max(b);
        return;
    }

    if index % 2 == 0 {
        for (l, used_l) in used.iter().copied().enumerate() {
            if used_l {
                continue;
            }

            pairs.push((l, 2 * n));
            used[l] = true;
            dfs(n, a, ans, pairs, used, index + 1);
            pairs.pop();
            used[l] = false;
            break;
        }
    } else {
        let l = pairs.pop().unwrap().0;
        for r in l + 1..2 * n {
            if used[r] {
                continue;
            }
            pairs.push((l, r));
            used[r] = true;
            dfs(n, a, ans, pairs, used, index + 1);
            pairs.pop();
            used[r] = false;
        }
        pairs.push((l, 2 * n));
    }
}

fn main() {
    input! {
        n: usize,
    };
    let a = {
        let mut a = vec![];
        for i in 0..2 * n {
            input! {
                a_i: [usize; 2 * n - i - 1]
            }
            a.push(a_i);
        }
        let mut a2 = vec![vec![0; 2 * n]; 2 * n];
        for i in 0..2 * n {
            for j in 0..2 * n - i - 1 {
                a2[i][i + 1 + j] = a[i][j];
                a2[i + 1 + j][i] = a[i][j];
            }
        }
        a2
    };

    let mut ans = 0_usize;
    let mut pairs = vec![];
    let mut used = vec![false; 2 * n];
    dfs(n, &a, &mut ans, &mut pairs, &mut used, 0);

    println!("{}", ans);
}
