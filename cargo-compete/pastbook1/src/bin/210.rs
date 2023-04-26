use proconio::input;

fn f(max: &mut i64, n: usize, a: &Vec<Vec<i64>>, group: &mut Vec<usize>, i: usize) {
    if i == n {
        let mut sum = 0_i64;
        for i in 0..n {
            for j in 0..n {
                if i == j || group[i] != group[j] {
                    continue;
                }
                sum += a[i][j];
            }
        }
        *max = (*max).max(sum);
        return;
    }

    for k in 0..3 {
        group.push(k);
        f(max, n, a, group, i + 1);
        group.pop();
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec![0_i64; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            input! {
                a_ij: i64,
            }
            a[i][j] = a_ij;
        }
    }

    let mut ans = -(1_i64 << 60);
    let mut group = vec![];
    f(&mut ans, n, &a, &mut group, 0);
    println!("{}", ans);
}
