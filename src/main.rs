// Time Domain Reflectometry lattice diagram Analysis for GPU using PCIe 3.0 x8.

use ndarray::arr1;
use num_traits::pow;
use std::f64::consts::E;
use plotters::prelude::*;

fn main() {
    let z0: f64 = 43.43123; // Transmission line impedance
    let zi: f64 = 150.0; // Source impedance
    let zl: f64 = 160.0; // Load impedance
    let vi: f64 = 1.6; // Initial voltage
    let sq_er: f64 = sq_rt(3.9);
    let c: f64 = 2.99792458 * pow(10, 8) as f64; // Speed of electromagnetic wavelength propagating in a vacuum.
    let vp: f64 = c / sq_er; // Voltage phase
    const N_LANES: i8 = 8; // Number of lanes

    let d = arr1(&[61.667, 58.00, 58.00, 63.667, 60.333, 59.333, 58.667, 58.667]); // Length of lane (mm) (lane 1 - 8)
    let t = (d * sq_er) / vp; // Propagation delay
    let _rtd = &t * 2.0; // Round-trip delay

    let v1: f64 = (vi * z0) / (zi + z0); // Initial voltage step
    let pg: f64 = (zi - z0) / (zi + z0); // Internal voltage reflection coefficient
    let pt: f64 = (zl - z0) / (zl + z0); // Load voltage reflection coefficient

    let mut pgn: i8 = 0;
    let mut ptn: i8 = 0;

    // Since Z_GPU and Z_CPU are greater than Z_PCIe,
    // expect voltages at each end to increment monotonically
    // in decreasing steps towards 0.8258 Volts.

    let clane: i8 = 0;
    let mut _cycle: i8 = 0;
    let mut a: f64 = init_lane_vs(pg, pgn, pt, ptn, v1, t[clane as usize]);

    let vr_incre: f64 = 0.0;
    let vs_incre: f64 = 0.0;

    for clane in 0..=N_LANES-1 { // Current lane
        for _cycle in 0..=N_LANES-1 { // Current cycle

            // Receiving End
            let vr_incre = if _cycle == 0 {
                v_incre(pt, a) // initial receiving end voltage
            } else if _cycle < N_LANES {
                vr_incre + v_incre(pt, a) // Calculate new receiving end voltage
            } else {
                0.0
            };

            // Going from receiving end to sending end
            ptn += 1; // Collect Pt
            a = lane_vr(pg, pgn, pt, ptn, v1, t[clane as usize], clane); // out from receiving end.

            // Sending End
            let vs_incre = if _cycle == 0 {
                v1 + v_incre(pg, a) // initial sending end voltage
            } else if _cycle < N_LANES {
                vs_incre + v_incre(pg, a) // Calculate new sending end voltage
            } else {
                0.0
            };

            // Going from sending end to receiving end
            pgn += 1;
            a = lane_vs(pg, pgn, pt, ptn, v1, t[clane as usize], clane); // out from sending end.

            //println!("vr_incre: {:?}", vr_incre);
            //println!("vs_incre: {:?}", vs_incre);

        }
    }
}

fn sq_rt(input: f64) -> f64 {
    input.sqrt()
}

// Calculate initial voltage step for n lane.
fn init_lane_vs(pg: f64, pgn: i8, pt: f64, ptn: i8, v1: f64, t: f64) -> f64 {
    let a: f64 = pow(pg, pgn as usize) * pow(pt, ptn as usize) * v1 * pow(E, -t as usize);
    return a;
}

// Calculate increased voltage
fn v_incre(p: f64, a: f64) -> f64 {
    let v_incre: f64 = (1.0 + p) * a;
    return v_incre;
}

// Calculate receiver outgoing
fn lane_vr(pg: f64, pgn: i8, pt: f64, ptn: i8, v1: f64, t: f64, clane: i8) -> f64 {
    let a: f64 = pow(pg, pgn as usize)
        * pow(pt, ptn as usize)
        * v1
        * pow(E, (clane + 1) as usize * -t as usize);
    return a;
}

// Calculate sending outgoing
fn lane_vs(pg: f64, pgn: i8, pt: f64, ptn: i8, v1: f64, t: f64, clane: i8) -> f64 {
    let a: f64 =
        pow(pg, pgn as usize) * pow(pt, ptn as usize) * v1 * pow(E, clane as usize * -t as usize);
    return a;
}
