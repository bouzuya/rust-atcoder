use std::collections::{BTreeMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut count = 0;
    let mut map = BTreeMap::new();
    let mut xs = VecDeque::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                xs.push_back(x);
            }
            2 => {
                if count > 0 {
                    count -= 1;
                    let key = *map.keys().next().unwrap();
                    let value = map.get_mut(&key).unwrap();
                    *value -= 1;
                    if *value == 0 {
                        map.remove(&key);
                    }
                    println!("{}", key);
                    continue;
                }
                match xs.pop_front() {
                    None => {
                        let key = *map.keys().next().unwrap();
                        let value = map.get_mut(&key).unwrap();
                        *value -= 1;
                        if *value == 0 {
                            map.remove(&key);
                        }
                        println!("{}", key);
                    }
                    Some(x) => {
                        println!("{}", x);
                    }
                }
            }
            3 => {
                for x in xs {
                    *map.entry(x).or_insert(0) += 1;
                    count += 1;
                }
                xs = VecDeque::new();
            }
            _ => unreachable!(),
        }
    }
}
