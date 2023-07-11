use proconio::input;

// ARC002 A
fn is_leap_year(y: i64) -> bool {
    if y % 400 == 0 {
        return true;
    }
    if y % 100 == 0 {
        return false;
    }
    if y % 4 == 0 {
        return true;
    }
    return false;
}

fn main() {
    input! {
        ymd: String
    };
    let ymd = ymd.split('/').collect::<Vec<&str>>();
    let mut y = ymd[0].parse::<i64>().unwrap();
    let mut m = ymd[1].parse::<i64>().unwrap();
    let mut d = ymd[2].parse::<i64>().unwrap();
    let mds = vec![
        vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    ];
    let [y, m, d] = loop {
        if y % m == 0 && (y / m) % d == 0 {
            break [y, m, d];
        }
        let md = mds[if is_leap_year(y) { 1 } else { 0 }][(m - 1) as usize];
        if d + 1 <= md {
            d += 1;
        } else {
            d = 1;
            if m + 1 <= 12 {
                m += 1;
            } else {
                m = 1;
                y += 1;
            }
        }
    };
    println!("{:04}/{:02}/{:02}", y, m, d);
}
