use proconio::input;

const EPS: f64 = 1e-9;

fn main() {
    input! {
        n: usize,
        k: usize,
        xycv: [(f64, f64, f64); n]
    };
    if k == 1 {
        println!("{}", 0);
        return;
    }

    let mut l = 0_f64;
    let mut r = 100_000_000_f64;
    for _ in 0..100 {
        let t = (l + r) / 2_f64;
        if is_ok(&xycv, k, t) {
            r = t;
        } else {
            l = t;
        }
    }

    println!("{:.10}", r);
}

fn is_ok(xycv: &Vec<(f64, f64, f64)>, k: usize, t: f64) -> bool {
    let cv: Vec<C> = xycv
        .iter()
        .map(|&(x, y, c)| C::new(V::new(x, y), t / c))
        .collect();

    let mut vv = Vec::new();
    for i in 0..cv.len() {
        for j in 0..i {
            let iv = cv[i].xp(&cv[j]);
            vv.extend(iv);
        }
    }
    for c in cv.iter() {
        vv.push(c.o);
    }

    for &v in vv.iter() {
        let mut count = 0;

        for &c in cv.iter() {
            if (v - c.o).norm() <= c.r + EPS {
                count += 1
            }
        }

        if count >= k {
            return true;
        }
    }
    false
}

#[derive(Clone, Copy)]
struct C {
    o: V,
    r: f64,
}

impl C {
    fn new(o: V, r: f64) -> Self {
        Self { o, r }
    }

    fn xp(&self, c: &C) -> Vec<V> {
        let v = c.o - self.o;
        let l = v.norm();
        if l < EPS {
            return vec![];
        }
        // TODO: 直線になるケースが考慮されていない
        let x = (l * l - c.r * c.r + self.r * self.r) / (2.0 * l);
        let y = (self.r * self.r - x * x).sqrt();
        let m = self.o + v * (x / l);
        let rv = V::new(v.y, -v.x) * (y / l);
        vec![m + rv, m - rv]
    }
}

#[derive(Clone, Copy)]
struct V {
    x: f64,
    y: f64,
}

impl V {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl std::ops::Add<V> for V {
    type Output = V;
    fn add(self, rhs: V) -> V {
        V {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<V> for V {
    type Output = V;
    fn sub(self, rhs: V) -> V {
        V {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f64> for V {
    type Output = V;
    fn mul(self, rhs: f64) -> V {
        V {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<f64> for V {
    type Output = V;
    fn div(self, rhs: f64) -> V {
        V {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
