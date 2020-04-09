use proconio::input;

// 01BFS
fn shortest_path(wvv: &Vec<Vec<(usize, usize)>>, iv: usize, ev: usize) -> usize {
    let mut uv = vec![false; wvv.len()];
    let mut deque = std::collections::VecDeque::new();
    deque.push_back((iv, 0));
    while let Some((v, c)) = deque.pop_front() {
        if v == ev {
            return c;
        }
        uv[v] = true;
        for &(nv, w) in wvv[v].iter() {
            if uv[nv] {
                continue;
            }
            match w {
                0 => deque.push_front((nv, c + w)),
                1 => deque.push_back((nv, c + w)),
                _ => unreachable!(),
            }
        }
    }
    wvv.len()
}

fn main() {
    input! {
        k: usize
    };
    let wvv: Vec<Vec<(usize, usize)>> = (0..k)
        .map(|i| vec![((i + 1) % k, 1), ((i * 10) % k, 0)])
        .collect();
    println!("{}", shortest_path(&wvv, 1, 0) + 1);
}
