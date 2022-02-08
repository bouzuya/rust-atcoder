use proconio::input;

fn merge_sort(a: &mut Vec<usize>, l: usize, r: usize) {
    if r - l == 1 {
        return;
    }
    let n = r - l;

    let m = l + (r - l) / 2;
    merge_sort(a, l, m);
    merge_sort(a, m, r);

    let mut i_l = l;
    let mut i_r = m;
    let mut c = vec![0; n];
    for c_i in c.iter_mut() {
        match (i_l == m, i_r == r) {
            (false, false) => {
                if a[i_l] <= a[i_r] {
                    *c_i = a[i_l];
                    i_l += 1;
                } else {
                    *c_i = a[i_r];
                    i_r += 1;
                }
            }
            (true, false) => {
                *c_i = a[i_r];
                i_r += 1;
            }
            (false, true) => {
                *c_i = a[i_l];
                i_l += 1;
            }
            (true, true) => unreachable!(),
        }
    }

    for (i, c_i) in c.iter().copied().enumerate() {
        a[l + i] = c_i;
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    merge_sort(&mut a, 0, n);

    for (i, a_i) in a.iter().copied().enumerate() {
        print!("{}{}", a_i, if i == n - 1 { '\n' } else { ' ' });
    }
}
