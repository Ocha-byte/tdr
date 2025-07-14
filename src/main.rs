// Time Domain Reflectometry lattice diagram for GPU using PCIe 3.0 x8 Analysis.

use ndarray::arr1;
use num_traits::pow;
use std::f64::consts::E;

fn main() {
    let z0: f64 = 43.43123; // Transmission line impedance
    let zi: f64 = 150.0; // Source impedance
    let zl: f64 = 160.0; // Load impedance
    let vi: f64 = 1.6; // Initial voltage
    let sq_er: f64 = sq_rt(3.9);
    let c: f64 = 2.99792458 * pow(10, 8) as f64; // Speed of electromagnetic wavelength propagating in a vacuum.
    let vp: f64 = c / sq_er; // Voltage phase
    let n_lanes: i8 = 8;

    let d = arr1(&[61.667, 58.00, 58.00, 63.667, 60.333, 59.333, 58.667, 58.667]); // Length of lane (mm) (lane 1 - 8)
    let t = (d * sq_er) / vp; // Propagation delay
    let _rtd = &t * 2.0; // Round-trip delay

    //println!("Propagation Delay: {:?}", t);
    //println!("Round Trip Delay: {:?}", _rtd);

    let v1: f64 = (vi * z0) / (zi + z0); // Initial voltage step
    let pg: f64 = (zi - z0) / (zi + z0); // Internal voltage reflection coefficient
    let pt: f64 = (zl - z0) / (zl + z0); // Load voltage reflection coefficient

    let pgn: i8 = 1;
    let ptn: i8 = 1;

    // Calculate initial voltage step for each lane
    let _lane = 0;
    let mut clane: i8 = 0;

    for _lane in 0..=n_lanes - 1 {
        let t_temp = t[clane as usize];
        println!("t: {:?}", &t[clane as usize]);
        let _a = int_lane_vs(pg, pgn, pt, ptn, v1, t_temp);

        println!("Clane: {:?}", clane);

        if clane < n_lanes - 1 {
            clane += 1;
        }
    }
}

fn sq_rt(input: f64) -> f64 {
    input.sqrt()
}

// Calculate initial voltage step for n lane.
fn int_lane_vs(pg: f64, pgn: i8, pt: f64, ptn: i8, v1: f64, t: f64) -> f64 {
    let a: f64 = pow(pg, pgn as usize) * pow(pt, ptn as usize) * v1 * pow(E, -t as usize);
    return a;
}
