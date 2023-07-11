use std::cmp;

use proconio::input;

// WA
// fn main() {
//     input! {
//         n: usize,
//         xy: [(i64, i64); n],
//     };
//     let mut max_d = 0_f64;
//     let dir = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
//     for (dx, dy) in dir {
//         let mut x = 0;
//         let mut y = 0;
//         for (x_i, y_i) in xy.iter() {
//             if x_i * dx + y_i * dy < 0 {
//                 continue;
//             }
//             x += x_i;
//             y += y_i;
//         }
//         let d = ((x * x + y * y) as f64).sqrt();
//         max_d = if d > max_d { d } else { max_d };
//     }
//     let ans = max_d;
//     println!("{}", ans);
// }

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    };
    xy.sort_by(|&(x_i, y_i), &(x_j, y_j)| {
        let i = (y_i as f64).atan2(x_i as f64);
        let j = (y_j as f64).atan2(x_j as f64);
        i.partial_cmp(&j).unwrap()
    });

    xy.append(&mut xy.clone());

    let mut max_d2 = 0;
    for i in 0..n {
        for j in i..(i + n) {
            let mut x = 0;
            let mut y = 0;
            for k in i..=j {
                x += xy[k].0;
                y += xy[k].1;
            }
            max_d2 = cmp::max(max_d2, x * x + y * y);
        }
    }
    let ans = (max_d2 as f64).sqrt();
    println!("{}", ans);
}
