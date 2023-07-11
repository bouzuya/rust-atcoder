use proconio::input;

fn dfs(
    max: &mut usize,
    used: &mut Vec<bool>,
    pairs: &mut Vec<(usize, usize)>,
    n: usize,
    a: &[Vec<usize>],
    index: usize,
    i: usize,
) {
    if pairs.len() == n {
        let mut b = 0_usize;
        for (i, j) in pairs.iter().copied() {
            b ^= a[i][j];
        }
        *max = (*max).max(b);
        return;
    }

    if index % 2 == 0 {
        for i in 0..2 * n {
            if used[i] {
                continue;
            }
            used[i] = true;
            dfs(max, used, pairs, n, a, index + 1, i);
            used[i] = false;
            break;
        }
    } else {
        for j in i + 1..2 * n {
            if used[j] {
                continue;
            }
            pairs.push((i, j));
            used[j] = true;
            dfs(max, used, pairs, n, a, index + 1, i);
            pairs.pop();
            used[j] = false;
        }
    }
}

fn main() {
    input! {
        n: usize,
    };
    let mut a = vec![];
    for i in 1..=2 * n - 1 {
        input! {
            a_i: [usize; 2 * n - i]
        }
        let mut a_i2 = vec![0; 2 * n];
        for j in 0..2 * n - i {
            a_i2[j + i] = a_i[j];
        }
        a.push(a_i2);
    }

    let mut max = 0_usize;
    let mut used = vec![false; 2 * n];
    let mut pairs = vec![];
    dfs(&mut max, &mut used, &mut pairs, n, &a, 0, 0);
    let ans = max;
    println!("{}", ans);
}
