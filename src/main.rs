// Time Domain Reflectometry for GPU using PCIe 3.0 x8

use ndarray::arr1;
use num_traits::pow;

fn main() {
    let z0: f64 = 43.43123; // Transmission line impedance
    let zi: f64 = 150.0; // Source impedance
    let zl: f64 = 160.0; // Load impedance
    let vi: f64 = 1.6; // Initial voltage
    let er: f64 = 15.21; // Relative permittivity
    let sq_er: f64 = er.sqrt();
    let c: f64 = 2.99792458 * pow(10, 8) as f64; // Speed of electromagnetic wavelength propagating in a vacuum.
    let vp: f64 = c / sq_er; // Voltage peak?

    let d = arr1(&[61.667, 58.00, 58.00, 63.667, 60.333, 59.333, 58.667, 58.667]); // Length of lane (mm) (lane 1 - 8)
    let t = (d * sq_er) / vp; // Propagation delay
    let rtd = &t * 2.0; // Round-trip delay

    let v1: f64 = (vi * z0) / (zi + z0); // Initial voltage step
    let pg: f64 = (zi - z0) / (zi + z0); // Internal voltage reflection coefficient
    let pt: f64 = (zl - z0) / (zl + z0); // Load voltage reflection coefficient

    // Loop index for pg and pt
    let pgn: i8 = 0;
    let ptn: i8 = 0;

    // Calculate initial voltage step for each lane
    let a =
        pow(pg, pgn.try_into().unwrap()) * (pow(pt, ptn.try_into().unwrap())) * v1 * -1.0 * (&t);

    let mut vs_list: [f64; 8] = [v1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut vr_incre;
    let mut vs_incre;
    let mut vr_list;
    let mut pt_list; //: [f64; 8];
    let mut pg_list;
    let mut time: [f64; 8];

    for n in 1..=8 {
        // Receiving end
        vr_incre = (1.0 + pt) * a; // Incoming
        pt_list = a; // Store a

        // Store vr
        if n == 1 {
            vr_list = vr_incre; // Store initial vr
        } else {
            vr_list = vr_incre + vr_list[n - 1]; // Store incremented vr
        }

        ptn += 1; // Collect pt.
        a = pow(pg, pgn.try_into().unwrap()) // Outgoing
            * (pow(pt, ptn.try_into().unwrap()))
            * v1
            * -1.0
            * (n + 1) as f64
            * (&t);

        // Sending end
        vs_incre = (1.0 + pg) * a; // Incoming
        pg_list = a; // Store a

        // Store vs
        if n == 1 {
            vs_list = vs_incre + vs_list[1]; // Store initial vs
        } else {
            vs_list = vs_incre + vs_list[n]; // Store incremented vs
        }

        pgn += 1; // Collect pg
        a = pow(pg, pgn.try_into().unwrap()) // Outgoing
            * (pow(pt, ptn.try_into().unwrap()))
            * v1
            * -1.0
            * n as f64
            * (&t);

        time[n] = n as f64; // Store respective time.
    }

    /*
     * Ringing occurs when the round trip delay < rise time. Here, RTD > rise time.
     * If there are any timing converns with this arrangement,
     * the length of the PCIe data line or dielectric constant could be changed.
     */
    // Check for ringing

    /* Electromagnetic Compatibility Check.
     * Determine effect of static field.
     */
}
