use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    };

    let d = vec![
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];
    let mut xy3 = vec![];
    for (dx, dy) in d.iter().copied() {
        let x = x1 + dx;
        let y = y1 + dy;
        xy3.push((x, y));
    }
    for (dx, dy) in d.iter().copied() {
        let x = x2 + dx;
        let y = y2 + dy;
        if xy3.contains(&(x, y)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
