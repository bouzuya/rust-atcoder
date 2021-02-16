use proconio::input;

fn main() {
    input! {
        a: [[i64; 4]; 4],
    };
    for i in 0..4 {
        for j in 0..4 {
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dy, dx) in dir {
                let (ny, nx) = (i as i64 + dy, j as i64 + dx);
                if (0..4 as i64).contains(&ny) && (0..4 as i64).contains(&nx) {
                    if a[i][j] == a[ny as usize][nx as usize] {
                        println!("CONTINUE");
                        return;
                    }
                }
            }
        }
    }
    println!("GAMEOVER");
}
