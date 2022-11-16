use std::time::{Duration, Instant};

use proconio::input;
use rand::{thread_rng, Rng};

// 4625995259
#[allow(dead_code)]
fn f1(n: usize, _a: &[Vec<i64>]) -> Vec<(usize, usize, i64)> {
    let mut ans = vec![];
    let mut rng = thread_rng();
    for _ in 0..1000 {
        let i = rng.gen::<usize>() % n;
        let j = rng.gen::<usize>() % n;
        ans.push((i, j, 1));
    }
    ans
}

// 7995220589
#[allow(dead_code)]
fn f2(n: usize, a: &[Vec<i64>]) -> Vec<(usize, usize, i64)> {
    let get_score = |b: &[Vec<i64>]| -> i64 {
        let mut sum = 0_i64;
        for i in 0..n {
            for j in 0..n {
                sum += (a[i][j] - b[i][j]).abs();
            }
        }
        200_000_000 - sum
    };
    let change = |b: &mut Vec<Vec<i64>>,
                  ans: &mut Vec<(usize, usize, i64)>,
                  t: usize,
                  x: usize,
                  y: usize,
                  h: i64| {
        let (px, py, ph) = ans[t];
        let (px, py) = (px as i64, py as i64);
        for i in 0..n {
            for j in 0..n {
                b[i as usize][j as usize] -=
                    (ph - (px - i as i64).abs() - (py - j as i64).abs()).max(0);
            }
        }

        ans[t] = (x, y, h);
        let (x, y) = (x as i64, y as i64);
        for i in 0..n {
            for j in 0..n {
                b[i][j] += (h - (x - i as i64).abs() - (y - j as i64).abs()).max(0);
            }
        }
    };

    let mut ans = vec![];
    let mut b = vec![vec![0_i64; n]; n];
    let mut rng = thread_rng();
    for _ in 0..1000 {
        let i = rng.gen::<usize>() % n;
        let j = rng.gen::<usize>() % n;
        ans.push((i, j, 1));
        b[i][j] += 1;
    }

    let mut score = get_score(&b);
    let instant = Instant::now();
    let duration = Duration::new(5, 950_000_000);

    while instant.elapsed() < duration {
        let t = rng.gen::<usize>() % ans.len();
        let (pi, ni) = (ans[t].0, ans[t].0 as i64 + rng.gen_range(-9, 9));
        let (pj, nj) = (ans[t].1, ans[t].1 as i64 + rng.gen_range(-9, 9));
        let (ph, nh) = (ans[t].2, ans[t].2 + rng.gen_range(-19, 19));
        if !((0..n as i64).contains(&ni)
            && (0..n as i64).contains(&nj)
            && (1..=n as i64).contains(&nh))
        {
            continue;
        }

        change(&mut b, &mut ans, t, ni as usize, nj as usize, nh);
        let new_score = get_score(&b);

        if score < new_score {
            score = new_score;
        } else {
            change(&mut b, &mut ans, t, pi, pj, ph);
        }
    }

    ans
}

fn main() {
    let n = 100_usize;
    input! {
        a: [[i64; n]; n],
    };
    let ans = f2(n, &a);
    println!("{}", ans.len());
    for (x, y, h) in ans {
        println!("{} {} {}", x, y, h);
    }
}
