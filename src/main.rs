// Time Domain Reflectometry lattice diagram Analysis for GPU using PCIe 3.0 x8.

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
    const N_LANES: i8 = 8; // Number of lanes

    let d = arr1(&[61.667, 58.00, 58.00, 63.667, 60.333, 59.333, 58.667, 58.667]); // Length of lane (mm) (lane 1 - 8)
    let t = (d * sq_er) / vp; // Propagation delay
    let _rtd = &t * 2.0; // Round-trip delay

    let v1: f64 = (vi * z0) / (zi + z0); // Initial voltage step
    let pg: f64 = (zi - z0) / (zi + z0); // Internal voltage reflection coefficient
    let pt: f64 = (zl - z0) / (zl + z0); // Load voltage reflection coefficient

    let mut pgn: i8 = 0;
    let mut ptn: i8 = 0;

    /* Since Z_GPU and Z_CPU are greater than Z_PCIe,
     * expect voltages at each end to increment monotonically
     * in decreasing steps towards 0.8258 Volts.
     * */

    // Calculate initial voltage step for each lane
    let _lane = 0;
    let mut clane: i8 = 0;
    let mut a_prev: f64 = init_lane_vs(pg, pgn, pt, ptn, v1, t[clane as usize]);
    let mut a_curr: f64 = 0.0;
    let mut vr_incre: [f64; N_LANES as usize] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut vs_incre: [f64; N_LANES as usize] = [v1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    let vr_incre_prev: f64 = 0.0;
    let vr_incre_curr: f64 = 0.0;
    let vs_incre_prev: f64 = 0.0;
    let vs_incre_curr: f64 = 0.0;
    let mut time: [i8; N_LANES as usize] = [0, 0, 0, 0, 0, 0, 0, 0];

    for _lane in 0..=N_LANES - 1 {
        // Calculate receiving end
        if _lane == 0 {
            let vr_incre_curr: f64 = v_incre(pt, a_prev); // initial receiving end
            vr_incre[_lane as usize] = vr_incre_curr; // Store result
        } else if _lane < N_LANES - 1 {
            a_prev = a_curr;
            let vr_incre_prev = vr_incre_curr; // set current to previous receiving end
            let vr_incre_curr: f64 = vr_incre_prev + v_incre(pt, a_prev); // Calculate new receiving end.
            vr_incre[_lane as usize] = vr_incre_curr; // Store result
        } else {
            a_prev = 0.0;
            let vr_incre_curr: f64 = vr_incre_prev + v_incre(pt, a_prev); // No change.
            vr_incre[_lane as usize] = vr_incre_curr; // Store result
        }

        // Collect Pt
        ptn += 1;
        a_curr = lane_vr(pg, pgn, pt, ptn, v1, t[clane as usize], clane); // out from receiving end.

        // Calculate sending end
        if _lane == 0 {
            let vs_incre_curr: f64 = v_incre(pg, a_curr);
            vs_incre[(_lane + 1) as usize] = vs_incre_curr + vs_incre[_lane as usize];
        } else if _lane < N_LANES - 1 {
            let vs_incre_prev = vs_incre_curr; // set current to previous sending end
            let vs_incre_curr = vs_incre_prev + v_incre(pg, a_curr);
            vs_incre[(_lane + 1) as usize] = vs_incre_curr;
        } else if _lane == N_LANES {
            // Do nothing.
            a_prev = 0.0;
            let vs_incre_curr = vs_incre_prev + v_incre(pg, a_prev);
            vs_incre[_lane as usize] = vs_incre_curr;
        }

        // Collect Pg
        pgn += 1;
        a_curr = lane_vs(pg, pgn, pt, ptn, v1, t[clane as usize], clane); // out from sending end.
        time[_lane as usize] = _lane;

        // Calculate next lane.
        if clane < N_LANES - 1 {
            clane += 1;
        }
    }
    println!("vr_incre: {:?}", vr_incre);
    println!("vs_incre: {:?}", vs_incre);
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
