use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        a: [usize; n],
    };

    let b = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();

    // println!("a = {:?}", a);
    // println!("b = {:?}", b);

    let s = p + q + r;
    for x in 0..n {
        match b[x..].binary_search_by_key(&s, |b_w| b_w - b[x]) {
            Err(_) => continue,
            Ok(i_w) => {
                let w = x + i_w;
                // println!("x = {} w = {}", x, w);
                match b[x..w].binary_search_by_key(&p, |b_y| b_y - b[x]) {
                    Err(_) => continue,
                    Ok(i_y) => {
                        let y = x + i_y;
                        // println!("x = {} y = {} w = {}", x, y, w);
                        match b[y..w].binary_search_by_key(&q, |b_z| b_z - b[y]) {
                            Err(_) => continue,
                            Ok(i_z) => {
                                let z = y + i_z;
                                if x != y && y != z && z != w {
                                    if b[y] - b[x] == p && b[z] - b[y] == q && b[w] - b[z] == r {
                                        // println!("x = {} y = {} z = {} w = {}", x, y, z, w);
                                        println!("Yes");
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
