extern crate rayon;
extern crate num_traits;
extern crate generic_array;
extern crate numeric_array;

use rayon::prelude::*;
use std::io::Write;
use numeric_array::NumericArray as Arr;
use generic_array::typenum::consts::U8;

// [f64;8]
type Vecf64 = Arr<f64, U8>;

const MAX_ITER: usize = 50;
const VLEN: usize = 8;

pub fn mbrot8(cr: Vecf64, ci: Vecf64) -> u8 {
    let mut zr = Arr::from_element(0f64);
    let mut zi = Arr::from_element(0f64);
    let mut tr = Arr::from_element(0f64);
    let mut ti = Arr::from_element(0f64);
    let mut absz = Arr::from_element(0f64);

    for _ in 0..MAX_ITER / 5 {
        for _ in 0..5 {
            zi = (zr + zr) * zi + ci;
            zr = tr - ti + cr;
            tr = zr * zr;
            ti = zi * zi;
        }

        absz = tr + ti;
        if absz.iter().all(|&t| t > 4.) {
            return 0;
        }
    }

    absz
        .iter()
        .enumerate()
        .fold(0, |accu, (i, &t)| accu | if t <= 4. { 0x80 >> i } else { 0 })
}

fn main() {
    let size = std::env::args()
        .nth(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(100);
    // Round size to multiple of 8
    let size = size / VLEN * VLEN;

    let inv = 2. / size as f64;

    let mut xloc = vec![Arr::from_element(0f64); size / VLEN];
    for i in 0..size {
        xloc[i / VLEN][i % VLEN] = i as f64 * inv - 1.5;
    }

    let stdout_unlocked = std::io::stdout();
    // Main thread only can print to stdout
    let mut stdout = stdout_unlocked.lock();

    println!("P4\n{} {}", size, size);

    let rows: Vec<Vec<u8>> = (0..size)
        .into_par_iter()
        .map(|y| {
            let ci = Arr::from_element(y as f64 * inv - 1.);
            (0..size / VLEN).map(|x| mbrot8(xloc[x], ci)).collect()
        })
        .collect();

    for row in rows {
        let _ = stdout.write_all(&row);
    }
}