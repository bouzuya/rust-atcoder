use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    };
    let h_t = h + 2;
    let w_t = w + 2;
    let mut t = vec![];
    t.push((0..w_t).map(|_| '#').collect());
    t.append(
        &mut s
            .iter_mut()
            .map(|line| {
                let mut l: Vec<char> = vec![];
                l.push('#');
                l.append(line);
                l.push('#');
                l
            })
            .collect::<Vec<Vec<char>>>(),
    );
    t.push((0..w_t).map(|_| '#').collect());

    let d = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut ans = vec![vec![false; w_t]; h_t];
    for y in 1..h_t - 1 {
        for x in 1..w_t - 1 {
            if d.iter()
                .all(|&(d_y, d_x)| t[(y as i64 + d_y) as usize][(x as i64 + d_x) as usize] == '#')
            {
                ans[y][x] = true;
            }
        }
    }
    let mut u = vec![vec!['.'; w_t]; h_t];
    for x in 0..w_t {
        u[0][x] = '#';
        u[h_t - 1][x] = '#';
    }
    for y in 1..h_t - 1 {
        u[y][0] = '#';
        u[y][w_t - 1] = '#';
        for x in 1..w_t - 1 {
            if ans[y][x] {
                for &(d_y, d_x) in d.iter() {
                    u[(y as i64 + d_y) as usize][(x as i64 + d_x) as usize] = '#';
                }
            }
        }
    }
    if t == u {
        println!("possible");
        for y in 1..h_t - 1 {
            for x in 1..w_t - 1 {
                print!("{}", if ans[y][x] { '#' } else { '.' });
            }
            println!();
        }
    } else {
        println!("impossible");
    }
}
