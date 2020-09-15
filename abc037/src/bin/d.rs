use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
    };
    let mut e_i = vec![vec![]; w * h];
    let mut e_o = vec![vec![]; w * h];
    for r in 0..h {
        for c in 0..w {
            let v = a[r][c];
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (d_r, d_c) in dir.iter() {
                let (r_n, c_n) = (r as i64 + d_r, c as i64 + d_c);
                if (0..h as i64).contains(&r_n) && (0..w as i64).contains(&c_n) {
                    let (r_n, c_n) = (r_n as usize, c_n as usize);
                    let v_n = a[r_n][c_n];
                    if v_n < v {
                        e_i[r * w + c].push(r_n * w + c_n);
                    } else if v_n > v {
                        e_o[r * w + c].push(r_n * w + c_n);
                    }
                }
            }
        }
    }
    let mut b = vec![];
    for r in 0..h {
        for c in 0..w {
            b.push((a[r][c], r, c));
        }
    }
    b.sort_by_key(|(v, _, _)| -v);
    let mut done = vec![false; h * w];
    let mut d = vec![0; h * w];
    for &(_, br, bc) in b.iter() {
        if done[br * w + bc] {
            continue;
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back(br * w + bc);
        while let Some(p) = q.pop_front() {
            done[p] = true;
            d[p] += 1;
            d[p] %= 1_000_000_007;
            for &p_i in e_i[p].iter() {
                d[p_i] += d[p];
                d[p_i] %= 1_000_000_007;
                for j in 0..e_o[p_i].len() {
                    if e_o[p_i][j] == p {
                        e_o[p_i].remove(j);
                        break;
                    }
                }
                if e_o[p_i].is_empty() {
                    q.push_back(p_i);
                }
            }
        }
    }
    let mut ans = 0;
    for &d_i in d.iter() {
        ans += d_i;
        ans %= 1_000_000_007;
    }
    println!("{}", ans);
}
