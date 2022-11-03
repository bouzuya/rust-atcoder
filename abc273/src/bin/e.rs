use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut pages: HashMap<i64, Option<usize>> = HashMap::new();
    let mut nodes: Vec<(Option<usize>, i64)> = vec![];
    let mut tail: Option<usize> = None;
    for _ in 0..q {
        input! {
            t: String,
        }
        match t.as_str() {
            "ADD" => {
                input! {
                    x: i64,
                }
                nodes.push((tail, x));
                tail = Some(nodes.len() - 1);
            }
            "DELETE" => {
                if let Some(x) = tail {
                    tail = nodes[x].0;
                }
            }
            "SAVE" => {
                input! {
                    y: i64,
                }
                pages.insert(y, tail);
            }
            "LOAD" => {
                input! {
                    z: i64,
                }
                tail = match pages.get(&z) {
                    None => None,
                    Some(&p) => match p {
                        None => None,
                        Some(p) => Some(p),
                    },
                };
            }
            _ => unreachable!(),
        }

        println!("{}", tail.map(|x| nodes[x].1).unwrap_or(-1));
    }
}
