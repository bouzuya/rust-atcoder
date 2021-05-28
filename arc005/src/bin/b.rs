use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        x: Usize1,
        y: Usize1,
        w: String,
        c: [Chars; 9],
    };
    let (mut dy, mut dx) = match w.as_str() {
        "R" => (0, 1),
        "L" => (0, -1),
        "U" => (-1, 0),
        "D" => (1, 0),
        "RU" => (-1, 1),
        "RD" => (1, 1),
        "LU" => (-1, -1),
        "LD" => (1, -1),
        _ => unreachable!(),
    };
    let mut ans = vec![];
    let mut cur = (y, x);
    for _ in 0..4 {
        let (y, x) = cur;
        ans.push(c[y][x]);
        let (mut ny, mut nx) = (y as i64 + dy, x as i64 + dx);
        if ny < 0 {
            ny = 0 + 1;
            dy *= -1;
        } else if ny >= 9 {
            ny = 9 - 1 - 1;
            dy *= -1;
        }
        if nx < 0 {
            nx = 0 + 1;
            dx *= -1;
        } else if nx >= 9 {
            nx = 9 - 1 - 1;
            dx *= -1;
        }
        cur = (ny as usize, nx as usize);
    }
    println!("{}", ans.iter().collect::<String>());
}
