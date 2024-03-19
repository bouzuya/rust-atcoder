use std::collections::HashMap;

use proconio::input;

fn f(x: usize, y: usize, map: &mut HashMap<(usize, usize), usize>, c: usize, n: usize) {
    if map.get(&(c, n)).is_some() {
        return;
    }

    if n >= 2 {
        if c == 1 {
            f(x, y, map, 1, n - 1);
            f(x, y, map, 0, n);
            map.insert(
                (c, n),
                map.get(&(1, n - 1)).unwrap_or(&0) + map.get(&(0, n)).unwrap_or(&0) * x,
            );
        } else {
            f(x, y, map, 0, n - 1);
            f(x, y, map, 1, n - 1);
            map.insert(
                (c, n),
                map.get(&(1, n - 1)).unwrap_or(&0) + map.get(&(0, n - 1)).unwrap_or(&0) * y,
            );
        }
    }
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    };
    let mut map = HashMap::new();
    map.insert((1, 1), 0);
    map.insert((0, 1), 1);
    f(x, y, &mut map, 1, n);
    let ans = map.get(&(1, n)).unwrap();
    println!("{}", ans);
}
