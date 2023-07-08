use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };

    let mut cur = (0_usize, 0_usize);
    let mut pos = vec![];
    let dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dr, dc) in dir {
        for _ in 0..n - 1 {
            let (i, j) = cur;
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            let (nr, nc) = (nr as usize, nc as usize);
            pos.push((nr, nc));
            cur = (nr, nc);
        }
    }

    let mut b = a.clone();

    let mut cur = 0;
    for _ in 0..pos.len() {
        let next = (cur + 1) % pos.len();
        b[pos[next].0][pos[next].1] = a[pos[cur].0][pos[cur].1];
        cur += 1;
        cur %= pos.len();
    }

    // println!("{:?}", pos);
    // println!("{:?}", pos.len());
    for i in 0..n {
        for j in 0..n {
            print!("{}", b[i][j]);
        }
        println!();
    }
}
