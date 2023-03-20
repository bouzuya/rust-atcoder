use proconio::input;

fn main() {
    input! {
        _l: usize,
        n_1: usize,
        n_2: usize,
        vl_1: [(usize, usize); n_1],
        vl_2: [(usize, usize); n_2],
    };
    let (vl_1, vl_2) = {
        let mut vl1 = vec![];
        let mut count = 0_usize;
        for (v_1, l_1) in vl_1 {
            vl1.push((v_1, count + l_1));
            count += l_1;
        }
        let mut vl2 = vec![];
        let mut count = 0_usize;
        for (v_2, l_2) in vl_2 {
            vl2.push((v_2, count + l_2));
            count += l_2;
        }
        (vl1, vl2)
    };

    let (vl_1, vl_2) = {
        let mut vl1 = vec![];
        let mut vl2 = vec![];
        let mut i_1 = 0_usize;
        let mut i_2 = 0_usize;
        while i_1 < n_1 && i_2 < n_2 {
            let (v_1, l_1) = vl_1[i_1];
            let (v_2, l_2) = vl_2[i_2];
            match l_1.cmp(&l_2) {
                std::cmp::Ordering::Less => {
                    vl1.push((v_1, l_1));
                    vl2.push((v_2, l_1));
                    i_1 += 1;
                }
                std::cmp::Ordering::Equal => {
                    vl1.push((v_1, l_1));
                    vl2.push((v_2, l_2));
                    i_1 += 1;
                    i_2 += 1;
                }
                std::cmp::Ordering::Greater => {
                    vl1.push((v_1, l_2));
                    vl2.push((v_2, l_2));
                    i_2 += 1;
                }
            }
        }
        if i_1 < n_1 {
            vl1.push(vl_1[i_1]);
        }
        if i_2 < n_2 {
            vl2.push(vl_2[i_2]);
        }
        // println!("{:?}", vl_1);
        // println!("{:?}", vl_2);
        // println!("{:?}", vl1);
        // println!("{:?}", vl2);
        (vl1, vl2)
    };

    let mut count = 0_usize;
    let mut prev = 0_usize;
    for ((v1, l), (v2, _)) in vl_1.into_iter().zip(vl_2.into_iter()) {
        if v1 == v2 {
            count += l - prev;
        }
        prev = l;
    }

    let ans = count;
    println!("{}", ans);
}
