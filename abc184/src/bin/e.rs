use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h]
    };
    let mut s = (h, w);
    let mut g = (h, w);
    let mut t = vec![vec![]; 26];
    for i in 0..h {
        for j in 0..w {
            match a[i][j] {
                'S' => s = (i, j),
                'G' => g = (i, j),
                '.' | '#' => {}
                c => {
                    let k = (c as u8 - 'a' as u8) as usize;
                    t[k].push((i, j));
                }
            }
        }
    }
    let mut b = vec![None; 26];
    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    let mut d = vec![vec![None; w]; h];
    let mut q = std::collections::VecDeque::new();
    q.push_front((0, s.0, s.1));
    d[s.0][s.1] = Some(0);
    while let Some((d_i, h_i, w_i)) = q.pop_front() {
        if d[h_i][w_i].is_some() && d_i > d[h_i][w_i].unwrap() {
            continue;
        }
        if a[h_i][w_i] == 'G' {
            continue;
        }
        for &(dy, dx) in dir.iter() {
            let (ny, nx) = (h_i as i64 + dy, w_i as i64 + dx);
            if (0..h as i64).contains(&ny) && (0..w as i64).contains(&nx) {
                let (ny, nx) = (ny as usize, nx as usize);
                if a[ny][nx] == '#' {
                    continue;
                }
                if d[ny][nx].is_none() || d[ny][nx].unwrap() > d_i + 1 {
                    d[ny][nx] = Some(d_i + 1);
                    q.push_back((d_i + 1, ny, nx));
                }
            }
        }
        if a[h_i][w_i] != 'S' && a[h_i][w_i] != 'G' && a[h_i][w_i] != '.' && a[h_i][w_i] != '#' {
            let index = (a[h_i][w_i] as u8 - 'a' as u8) as usize;
            if b[index].is_none() || b[index].unwrap() > d_i + 1 {
                b[index] = Some(d_i + 1);
                for &(t_h_i, t_w_i) in t[index].iter() {
                    if t_h_i == h_i && t_w_i == w_i {
                        continue;
                    }
                    if d[t_h_i][t_w_i].is_none() || d[t_h_i][t_w_i].unwrap() > d_i + 1 {
                        d[t_h_i][t_w_i] = Some(d_i + 1);
                        q.push_front((d_i + 1, t_h_i, t_w_i));
                    }
                }
            }
        }
    }
    let ans = d[g.0][g.1].unwrap_or(-1);
    println!("{}", ans);
}
