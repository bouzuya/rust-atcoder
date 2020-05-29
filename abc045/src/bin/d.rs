use proconio::input;

fn f(
    h: i64,
    w: i64,
    set: &std::collections::BTreeSet<(i64, i64)>,
    d: &Vec<(i64, i64)>,
    p: (isize, isize),
) -> usize {
    let mut count = 0;
    for &(dx, dy) in d.iter() {
        let (nx, ny) = (p.0 as i64 + dx, p.1 as i64 + dy);
        if (1..=w).contains(&nx) && (1..=h).contains(&ny) {
            if set.contains(&(nx, ny)) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    input! {
        h: i64,
        w: i64,
        n: usize,
        ab: [(i64, i64); n]
    };
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
    let mut set = std::collections::BTreeSet::new();
    let mut c = vec![0; 9 + 1];
    c[0] = (h - 2) * (w - 2);
    for &(y, x) in ab.iter() {
        for &(dx, dy) in d.iter() {
            let (nx, ny) = (x as i64 + dx, y as i64 + dy);
            if (2..=w - 1).contains(&nx) && (2..=h - 1).contains(&ny) {
                let count = f(h, w, &set, &d, (nx as isize, ny as isize));
                c[count] -= 1;
                c[count + 1] += 1;
            }
        }
        set.insert((x, y));
    }

    for &c_i in c.iter() {
        println!("{}", c_i);
    }
}
