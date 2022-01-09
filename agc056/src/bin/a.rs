use std::collections::VecDeque;

use proconio::input;

fn ok(a: &[Vec<char>]) -> bool {
    let n = a.len();
    for i in 0..n {
        let mut count = 0;
        for j in 0..n {
            if a[i][j] == '#' {
                count += 1;
            }
        }
        if count != 3 {
            return false;
        }
    }
    for i in 0..n {
        let mut count = 0;
        for j in 0..n {
            if a[j][i] == '#' {
                count += 1;
            }
        }
        if count != 3 {
            return false;
        }
    }
    let mut count = 0;
    let mut used = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == '#' && !used[i][j] {
                count += 1;
                // bfs
                let mut deque = VecDeque::new();
                used[i][j] = true;
                deque.push_back((i, j));
                while let Some((r, c)) = deque.pop_front() {
                    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                    for (dr, dc) in dir {
                        let (nr, nc) = (r as i64 + dr, c as i64 + dc);
                        if !(0..n as i64).contains(&nr) || !(0..n as i64).contains(&nc) {
                            continue;
                        }
                        let (nr, nc) = (nr as usize, nc as usize);
                        if a[nr][nc] == '#' && !used[nr][nc] {
                            used[nr][nc] = true;
                            deque.push_back((nr, nc));
                        }
                    }
                }
            }
        }
    }
    count == n
}

fn main() {
    input! {
        n: usize,
    };
    let mut a = vec![vec!['.'; n]; n];
    for (i, a_i) in a.iter_mut().enumerate() {
        for j in 0..3 {
            a_i[(i * 3 + j) % n] = '#';
        }
    }

    if n % 3 != 0 {
        a.swap(0, n / 3 - 1);
        a.swap(n - n / 3, n - 1);
    }

    assert!(ok(&a));

    for a_i in a {
        println!("{}", a_i.iter().collect::<String>());
    }
}
