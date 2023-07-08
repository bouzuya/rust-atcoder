use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let board = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == a {
                let dir = vec![(0, -1), (0, 1)];
                for (dr, dc) in dir {
                    let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                    if !(0..3 as i64).contains(&nr) || !(0..3 as i64).contains(&nc) {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if board[nr][nc] == b {
                        println!("Yes");
                        return;
                    }
                }
                println!("No");
                return;
            }
        }
    }
}
