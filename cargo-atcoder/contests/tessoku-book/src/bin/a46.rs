use proconio::input;
use rand::{thread_rng, Rng};

// 貪欲法
#[allow(dead_code)]
fn f1(n: usize, xy: &[(i64, i64)]) -> Vec<usize> {
    let mut used = vec![false; n];
    used[0] = true;
    let mut ans = vec![0];
    let mut prev = 0;
    loop {
        let mut next = prev;
        let mut dist = 1_000_000_000_i64;
        let mut count = 0_usize;
        for i in 0..n {
            if used[i] {
                continue;
            }
            count += 1;

            let (px, py) = xy[prev];
            let (nx, ny) = xy[i];
            let d = (px - nx).pow(2) + (py - ny).pow(2);
            if d < dist {
                next = i;
                dist = d;
            }
        }
        if count == 0 {
            break;
        }

        used[next] = true;
        ans.push(next);
        prev = next;
    }

    ans.push(0);
    ans
}

// 局所探索法
#[allow(dead_code)]
fn f2(n: usize, xy: &[(i64, i64)]) -> Vec<usize> {
    let dist = |i: usize, j: usize| -> i64 {
        let (x1, y1) = xy[i];
        let (x2, y2) = xy[j];
        (x1 - x2).pow(2) + (y1 - y2).pow(2)
    };

    let score = |a: &[usize]| -> i64 {
        let mut sum = 0_i64;
        for i in 0..n - 1 {
            sum += dist(a[i], a[i + 1]);
        }
        sum
    };

    let reverse = |a: &mut Vec<usize>, mut l: usize, mut r: usize| {
        while l < r {
            a.swap(l, r);
            l += 1;
            r = r.saturating_sub(1);
        }
    };

    let mut rng = thread_rng();
    let mut ans = (0..n).collect::<Vec<usize>>();
    let mut cur = score(&ans);
    for _ in 0..200_000 {
        let l = 1 + rng.gen::<usize>() % (n - 1);
        let r = 1 + rng.gen::<usize>() % (n - 1);
        let (l, r) = if l > r { (r, l) } else { (l, r) };
        reverse(&mut ans, l, r);
        let new = score(&ans);
        if new < cur {
            cur = new;
        } else {
            reverse(&mut ans, l, r);
        }
    }

    ans.push(0);
    ans
}

// 焼きなまし法
#[allow(dead_code)]
fn f3(n: usize, xy: &[(i64, i64)]) -> Vec<usize> {
    let dist = |i: usize, j: usize| -> f64 {
        let (x1, y1) = xy[i];
        let (x2, y2) = xy[j];
        (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
    };

    let score = |a: &[usize]| -> f64 {
        let mut sum = 0_f64;
        for i in 0..n - 1 {
            sum += dist(a[i], a[i + 1]);
        }
        sum
    };

    let reverse = |a: &mut Vec<usize>, mut l: usize, mut r: usize| {
        while l < r {
            a.swap(l, r);
            l += 1;
            r = r.saturating_sub(1);
        }
    };

    let mut rng = thread_rng();
    let mut ans = (0..n).collect::<Vec<usize>>();
    let mut cur = score(&ans);
    for i in 0..200_000 {
        let l = 1 + rng.gen::<usize>() % (n - 1);
        let r = 1 + rng.gen::<usize>() % (n - 1);
        let (l, r) = if l > r { (r, l) } else { (l, r) };
        reverse(&mut ans, l, r);
        let new = score(&ans);

        // 温度
        let t = 30_f64 - 28_f64 * i as f64 / 200_000_f64;
        // 採用確率
        let p = 0_f64.min((cur - new) / t).exp();
        if rng.gen_range(0_f64, 1_f64) < p {
            cur = new;
        } else {
            reverse(&mut ans, l, r);
        }
    }

    ans.push(0);
    ans
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    // let ans = f1(n, &xy); // 4230
    // let ans = f2(n, &xy); // 4486
    let ans = f3(n, &xy); // 4874
    for a in ans {
        println!("{}", a + 1);
    }
}
