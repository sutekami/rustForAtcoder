use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [[usize; w]; h],
    }
    let mut ss: Vec<Vec<usize>> = vec![];
    let mut x: Vec<usize> = vec![];
    let mut y: Vec<usize> = vec![];
    for i in 0..h {
        let mut a: usize = 0;
        for j in 0..w {
            a += s[i][j];
        }
        y.push(a);
    }

    for i in 0..w {
        let mut a: usize = 0;
        for j in 0..h {
            a += s[j][i];
        }
        x.push(a);
    }
    for i in 0..h {
        let mut a: Vec<usize> = vec![];
        for j in 0..w {
            a.push(y[i] + x[j] - s[i][j]);
        }
        ss.push(a);
    }
    for i in &ss {
        println!("{:?}", i);
    }
}
