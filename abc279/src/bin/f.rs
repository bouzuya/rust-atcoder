// TLE
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut queries = vec![];
    for i in 0..n {
        queries.push((2, i, 0));
    }
    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                queries.push((1, x, y));
            }
            2 => {
                input! {
                    x: Usize1,
                }
                queries.push((2, x, 0));
            }
            3 => {
                input! {
                    x: Usize1,
                }
                queries.push((3, x, 0));
            }
            _ => unreachable!(),
        }
    }

    let m = 10 * q;
    let mut balls = vec![];
    let mut nodes = (m..m + n).collect::<Vec<usize>>();
    let mut box_node_index = (0..n).collect::<Vec<usize>>();
    for (t, x, y) in queries {
        match t {
            1 => {
                nodes[box_node_index[y]] = box_node_index[x];
                box_node_index[y] = nodes.len();
                nodes.push(m + y);
            }
            2 => {
                balls.push(box_node_index[x]);
            }
            3 => {
                let mut node_index = balls[x];
                let mut p = node_index;
                while node_index < m {
                    p = node_index;
                    node_index = nodes[node_index];
                }
                if p < m {
                    balls[x] = p;
                }
                let ans = node_index - m;
                println!("{}", ans + 1);
            }
            _ => unreachable!(),
        }
    }
}
