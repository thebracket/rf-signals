use num_complex::Complex;
mod helpers;
use helpers::*;
mod pure;
use pure::*;
mod prop;
use prop::*;

const THIRD : f64 = 1.0/3.0;

pub fn point_to_point(
    elev: &Vec<f64>,
    tht_m: f64,
    rht_m: f64,
    eps_dielect: f64,
    sgm_conductivity: f64,
    eno_ns_surfref: f64,
    frq_mhz: f64,
    radio_climate: i32,
    pol: i32,
    conf: f64,
    rel: f64,
    dbloss: &mut f64,
    strmode: &mut String,
    errnum: &mut i32
) {
    let mut prop = prop_type::default();
    let mut propv = propv_type::default();
    let mut propa = propa_type::default();

    let mut zsys = 0.0;
    let mut zc;
    let mut zr;
    let mut eno;
    let mut enso;
    let mut q;
    let mut ja;
    let mut jb;
    let mut np;
    /* double dkm, xkm; */
    let mut tpd;
    let mut fs;

    prop.hg[0] = tht_m;
    prop.hg[1] = rht_m;
    propv.klim = radio_climate;
    prop.kwx = 0;
    propv.lvar = 5;
    prop.mdp = -1;
    prop.ptx = pol;
    prop.thera = 0.0;
    prop.thenr = 0.0;
    zc = qerfi(conf);
    zr = qerfi(rel);
    np = elev[0] as i32;
    eno = eno_ns_surfref;
    enso = 0.0;
    q = enso;

    /* PRESET VALUES for Basic Version w/o additional inputs active */

    prop.encc = 1000.00;	/*  double enc_ncc_clcref preset  */
    prop.cch = 22.5;	/* double clutter_height preset to ILLR calibration.;  
           use 25.3 for ITU-P1546-2 calibration */
    prop.cd = 1.00;		/* double clutter_density preset */
    let mode_var = 1;	/* int mode_var set to 1 for FCC compatibility;
           normally, SPLAT presets this to 12 */
    prop.dhd = 0.0;		/* delta_h_diff preset */

    if q <= 0.0 {
        ja = (3.0 + 0.1 * elev[0]) as i32;
        jb = np - ja + 6;

        for i in (ja-1)..jb {
            zsys += elev[i as usize];
        }

        zsys /= (jb - ja + 1) as f64;
        q = eno;
    }

    propv.mdvar = mode_var;
    qlrps(frq_mhz, zsys, q, pol, eps_dielect, sgm_conductivity, &mut prop);
    qlrpfl2(elev, propv.klim, propv.mdvar, &mut prop, &mut propa, &mut propv);
tpd =
sqrt((prop.he[0] - prop.he[1]) * (prop.he[0] - prop.he[1]) +
 (prop.dist) * (prop.dist));
fs = 32.45 + 20.0 * log10(frq_mhz) + 20.0 * log10(tpd / 1000.0);
q = prop.dist - propa.dla;

if q.floor() < 0.0 {
    *strmode = "L-o-S".to_string();
} else {
if q.floor() == 0.0 {
    *strmode = "1_Hrzn".to_string();
}

else if q.floor() > 0.0 {
    *strmode = "2_Hrzn".to_string();
}

if prop.dist <= propa.dlsa || prop.dist <= propa.dx {

    if prop.dl[1].floor() == 0.0 {
        *strmode += "_Peak";
    }

    else {
        *strmode += "_Diff";
    }

} else if prop.dist > propa.dx {
    *strmode += "_Tropo";
}
}

*dbloss = unsafe { avar(zr, 0.0, zc, &mut prop, &mut propv) } + fs;
*errnum = prop.kwx;
}

fn qlrpfl2(pfl: &[f64], klimx: i32, mdvarx: i32, prop: &mut prop_type, propa: &mut propa_type, propv: &mut propv_type)
{
    let mut np;
    let mut xl = [0.0, 0.0];
    let mut dlb;
    let mut q = 0.0;
    let mut za = 0.0;
    let mut zb = 0.0;
    let mut temp;
    let mut rad;
    let mut rae1 = 0.0;
    let mut rae2 = 0.0;

    prop.dist = pfl[0] * pfl[1];
    np = pfl[0] as i32; // Is it doing this for a floor?
    hzns2(pfl, prop, propa);
    dlb = prop.dl[0] + prop.dl[1];
    prop.rch[0] = prop.hg[0] + pfl[2];
    prop.rch[1] = prop.hg[1] + pfl[np as usize + 2];

    for j in 0..2 {
        xl[j] = f64::min(15.0 * prop.hg[j], 0.1 * prop.dl[j]);
    }

    xl[1] = prop.dist - xl[1];
    prop.dh = d1thx2(pfl, xl[0], xl[1], propa);

    if (np < 1) || (pfl[1] > 150.0) {
    /* for TRANSHORIZON; diffraction over a mutual horizon, or for one or more obstructions */
    if dlb < 1.5 * prop.dist {
        z1sq2(pfl, xl[0], 0.9 * prop.dl[0], &mut za, &mut q);
        z1sq2(pfl, prop.dist - 0.9 * prop.dl[1], xl[1], &mut q, &mut zb);
        prop.he[0] = prop.hg[0] + fortran_dim(pfl[2], za);
        prop.he[1] = prop.hg[1] + fortran_dim(pfl[np as usize + 2], zb);
    }

    /* for a Line-of-Sight path */
    else {
        z1sq2(pfl, xl[0], xl[1], &mut za, &mut zb);
        prop.he[0] = prop.hg[0] + fortran_dim(pfl[2], za);
        prop.he[1] = prop.hg[1] + fortran_dim(pfl[np as usize + 2], zb);

        for j in 0..2 {
            prop.dl[j] =
                (2.0 * prop.he[j] / prop.gme).sqrt() *
                (-0.07 *
                (prop.dh / mymax(prop.he[j], 5.0)).sqrt()).exp();
        }

        /* for one or more obstructions only NOTE buried as in ITM FORTRAN and DLL, not functional  */
        if (prop.dl[0] + prop.dl[1]) <= prop.dist {
            /* q=pow(prop.dist/(dl[0]+dl[1])),2.0); */
            temp = prop.dist / (prop.dl[0] + prop.dl[1]);
            q = temp * temp;
        }

        for j in 0..2 {
            prop.he[j] *= q;
            prop.dl[j] =
                (2.0 * prop.he[j] / prop.gme).sqrt() *
                (-0.07 *
                (prop.dh / mymax(prop.he[j], 5.0)).sqrt()).exp();
        }

        /* this sets (or resets) prop.the, and is not in The Guide FORTRAN QLRPFL */
        for j in 0..2 {
            q = (2.0 * prop.he[j] / prop.gme).sqrt();
            prop.the[j] =
                (0.65 * prop.dh * (q / prop.dl[j] - 1.0) -
                    2.0 * prop.he[j]) / q;
        }
    }
    }

    else {			/* for ITWOM ,computes he for tx, rcvr, and the receiver approach angles for use in saalos */

    prop.he[0] = prop.hg[0] + (pfl[2]);
    prop.he[1] = prop.hg[1] + (pfl[np as usize + 2]);

    rad = prop.dist - 500.0;

    if prop.dist > 550.0 {
        z1sq2(pfl, rad, prop.dist, &mut rae1, &mut rae2);
    } else {
        rae1 = 0.0;
        rae2 = 0.0;
    }

    prop.thera = ((rae2 - rae1).abs() / prop.dist).atan();

    if rae2 < rae1 {
        prop.thera = -prop.thera;
    }

    prop.thenr =
        (mymax(0.0, pfl[np as usize + 2] - pfl[np as usize + 1]) / pfl[1]).atan();

    }

    prop.mdp = -1;
    propv.lvar = i32::max(propv.lvar, 3);

    if mdvarx >= 0 {
        propv.mdvar = mdvarx;
        propv.lvar = i32::max(propv.lvar, 4);
    }

    if klimx > 0 {
        propv.klim = klimx;
        propv.lvar = 5;
    }

    unsafe { lrprop2(0.0, prop, propa); }
}

// TODO: Add thread_local support
unsafe fn lrprop2(d: f64, prop: &mut prop_type, propa: &mut propa_type)
{
	/* ITWOM_lrprop2 */
    static mut wlos : bool = false;
    static mut wscat : bool = false;
    static mut dmin : f64 = 0.0;
    static mut xae : f64 = 0.0;
	//static thread_local bool wlos, wscat;
	//static thread_local double dmin, xae;
    let mut prop_zgnd = Complex::new(prop.zgndreal, prop.zgndimag);
    let mut pd1;
    let mut a0;
    let mut a1;
    let mut a2;
    let mut a3;
    let mut a4;
    let mut a5;
    let mut a6;
    let mut iw;
    let mut d0;
    let mut d1;
    let mut d2;
    let mut d3;
    let mut d4;
    let mut d5;
    let mut d6;
    let mut wq;
    let mut q;

	iw = prop.tiw;
	pd1 = prop.dist;
	propa.dx = 2000000.0;

	if prop.mdp != 0 {	/* if oper. mode is not 0, i.e. not area mode ongoing */
        for j in 0..2 {
			propa.dls[j] = (2.0 * prop.he[j] / prop.gme).sqrt();
        }

		propa.dlsa = propa.dls[0] + propa.dls[1];
		propa.dlsa = f64::min(propa.dlsa, 1000000.0);
		propa.dla = prop.dl[0] + prop.dl[1];
		propa.tha =
		    mymax(prop.the[0] + prop.the[1], -propa.dla * prop.gme);
		wlos = false;
		wscat = false;

		/*checking for parameters-in-range, error codes set if not */

		if prop.wn < 0.838 || prop.wn > 210.0 {
			prop.kwx = i32::max(prop.kwx, 1);
        }

        for j in 0..2 {
			if prop.hg[j] < 1.0 || prop.hg[j] > 1000.0 {
				prop.kwx = i32::max(prop.kwx, 1);
            }
        }

		if (prop.the[0]).abs() > 200e-3 {
			prop.kwx = i32::max(prop.kwx, 3);
        }

		if (prop.the[1]).abs() > 1.220 {
			prop.kwx = i32::max(prop.kwx, 3);
        }

		if prop.ens < 250.0 || prop.ens > 400.0 || prop.gme < 75e-9
		    || prop.gme > 250e-9
		    || prop_zgnd.re <= prop_zgnd.im.abs()
		    || prop.wn < 0.419 || prop.wn > 420.0 {
			prop.kwx = 4;
        }

        for j in 0..2 {
			if prop.hg[j] < 0.5 || prop.hg[j] > 3000.0 {
				prop.kwx = 4;
            }
        }

		dmin = (prop.he[0] - prop.he[1]).abs() / 200e-3;
		q = unsafe { adiff2(0.0, prop, propa) };
		xae = pow(prop.wn * (prop.gme * prop.gme), -THIRD);
		d3 = mymax(propa.dlsa, 1.3787 * xae + propa.dla);
		d4 = d3 + 2.7574 * xae;
		a3 = unsafe { adiff2(d3, prop, propa) };
		a4 = unsafe { adiff2(d4, prop, propa) };
		propa.emd = (a4 - a3) / (d4 - d3);
		propa.aed = a3 - propa.emd * d3;
	}

	if prop.mdp >= 0 {	/* if initializing the area mode */
		prop.mdp = 0;	/* area mode is initialized */
		prop.dist = d;
	}

	if prop.dist > 0.0 {
		if prop.dist > 1000e3 {	/* prop.dist being in meters, if greater than 1000 km, kwx=1 */
			prop.kwx = i32::max(prop.kwx, 1);
        }

		if prop.dist < dmin {
			prop.kwx = i32::max(prop.kwx, 3);
        }

		if prop.dist < 1e3 || prop.dist > 2000e3 {
			prop.kwx = 4;
        }
	}

	if prop.dist < propa.dlsa {

		if iw <= 0.0 {	/* if interval width is zero or less, used for area mode */

			if !wlos {
				q = alos2(0.0, prop, propa);
				d2 = propa.dlsa;
				a2 = propa.aed + d2 * propa.emd;
				d0 = 1.908 * prop.wn * prop.he[0] * prop.he[1];

				if propa.aed > 0.0 {
					prop.aref =
					    propa.aed + propa.emd * prop.dist;
				} else {
					if propa.aed == 0.0 {
						d0 = f64::min(d0, 0.5 * propa.dla);
						d1 = d0 + 0.25 * (propa.dla -
								  d0);
					} else {	/* aed less than zero */

						d1 = mymax(-propa.aed /
							   propa.emd,
							   0.25 * propa.dla);
					}
					a1 = alos2(d1, prop, propa);
					wq = false;

					if d0 < d1 {
						a0 = alos2(d0, prop, propa);
						a2 = f64::min(a2,
							   alos2(d2, prop,
								 propa));
						q = log(d2 / d0);
						propa.ak2 =
						    mymax(0.0,
							  ((d2 - d0) * (a1 -
									a0) -
							   (d1 - d0) * (a2 -
									a0)) /
							  ((d2 -
							    d0) * log(d1 / d0) -
							   (d1 - d0) * q));
						wq = propa.aed >= 0.0
						    || propa.ak2 > 0.0;

						if wq {
							propa.ak1 =
							    (a2 - a0 -
							     propa.ak2 * q) /
							    (d2 - d0);

							if propa.ak1 < 0.0 {
								propa.ak1 = 0.0;
								propa.ak2 =
								    fortran_dim
								    (a2,
								     a0) / q;

								if propa.ak2 ==
								    0.0 {
									propa.
									    ak1
									    =
									    propa.
									    emd;
                                    }
							}
						}
					}

					if !wq {
						propa.ak1 =
						    fortran_dim(a2,
								a1) / (d2 - d1);
						propa.ak2 = 0.0;

						if propa.ak1 == 0.0 {
							propa.ak1 = propa.emd;
                        }

					}
					propa.ael =
					    a2 - propa.ak1 * d2 -
					    propa.ak2 * log(d2);
					wlos = true;
				}
			}
		} else {	/* for ITWOM point-to-point mode */

			if !wlos {
				q = alos2(0.0, prop, propa);	/* coefficient setup */
				wlos = true;
			}

			if prop.los == 1 {	/* if line of sight */
				prop.aref = alos2(pd1, prop, propa);
			} else {
				if (prop.dist - prop.dl[0]) as i32 == 0 {	/* if at 1st horiz */
					prop.aref =
					    5.8 + alos2(pd1, prop, propa);
				} else if (prop.dist - prop.dl[0]).floor() > 0.0 {	/* if past 1st horiz */
					q = unsafe { adiff2(0.0, prop, propa) };
					prop.aref = unsafe { adiff2(pd1, prop, propa) };
				} else {
					prop.aref = 1.0;
				}

			}
		}
	}

	/* los and diff. range coefficents done. Starting troposcatter */
	if prop.dist <= 0.0 || prop.dist >= propa.dlsa {
		if iw == 0.0 {	/* area mode */
			if !wscat {
				q = ascat(0.0, prop, propa);
				d5 = propa.dla + 200e3;
				d6 = d5 + 200e3;
				a6 = ascat(d6, prop, propa);
				a5 = ascat(d5, prop, propa);

				if a5 < 1000.0 {
					propa.ems = (a6 - a5) / 200e3;
					propa.dx =
					    mymax(propa.dlsa,
						  mymax(propa.dla +
							0.3 * xae * log(47.7 *
									prop.
									wn),
							(a5 - propa.aed -
							 propa.ems * d5) /
							(propa.emd -
							 propa.ems)));

					propa.aes =
					    (propa.emd - propa.ems) * propa.dx +
					    propa.aed;
				}

				else {
					propa.ems = propa.emd;
					propa.aes = propa.aed;
					propa.dx = 10000000.0;
				}
				wscat = true;
			}

			if prop.dist > propa.dx {
				prop.aref = propa.aes + propa.ems * prop.dist;
			} else {
				prop.aref = propa.aed + propa.emd * prop.dist;
			}
		} else {	/* ITWOM mode  q used to preset coefficients with zero input */

			if !wscat {
				d5 = 0.0;
				d6 = 0.0;
				q = ascat(0.0, prop, propa);
				a6 = ascat(pd1, prop, propa);
				q = unsafe { adiff2(0.0, prop, propa) };
				a5 = unsafe { adiff2(pd1, prop, propa) };

				if a5 <= a6 {
					propa.dx = 10000000.0;
					prop.aref = a5;
				} else {
					propa.dx = propa.dlsa;
					prop.aref = a6;
				}
				wscat = true;
			}
		}
	}
	prop.aref = mymax(prop.aref, 0.0);
}

//TODO: thread local madness
unsafe fn adiff2(d: f64, prop: &mut prop_type, propa: &propa_type) -> f64
{
    let prop_zgnd = Complex::new(prop.zgndreal, prop.zgndimag);

    static mut wd1 : f64 = 0.0;
    static mut xd1 : f64  = 0.0;
    static mut qk : f64  = 0.0;
    static mut aht : f64  = 0.0;
    static mut xht : f64  = 0.0;
    static mut toh : f64  = 0.0;
    static mut toho : f64  = 0.0;
    static mut roh : f64  = 0.0;
    static mut roho : f64  = 0.0;
    static mut dto : f64  = 0.0;
    static mut dto1 : f64  = 0.0;
    static mut dtro : f64  = 0.0;
    static mut dro : f64  = 0.0;
    static mut dro2 : f64  = 0.0;
    static mut drto : f64  = 0.0;
    static mut dtr : f64  = 0.0;
    static mut dhh1 : f64  = 0.0;
    static mut dhh2 : f64  = 0.0;
    static mut dtof : f64  = 0.0;
    static mut dto1f : f64  = 0.0;
    static mut drof : f64 = 0.0;
    static mut dro2f : f64  = 0.0;
	/*static thread_local double wd1, xd1, qk, aht, xht, toh, toho, roh, roho, dto, dto1,
	    dtro, dro, dro2, drto, dtr, dhh1, dhh2, /* dhec, */ dtof, dto1f,
	    drof, dro2f;*/

    let mut a;
    let mut q;
    let mut pk;
    let mut rd;
    let mut ds;
    let mut dsl;
    let mut th;
    let mut wa;
    let mut sf2;
    let mut vv;
    let mut kedr = 0.0;
    let mut arp = 0.0;
    let mut sdr = 0.0;
    let mut pd = 0.0;
    let mut srp = 0.0;
    let mut kem = 0.0;
    let mut csd = 0.0;
    let mut sdl = 0.0;
    let mut adiffv2 = 0.0;
    let mut closs = 0.0;

	/* sf1=1.0; *//* average empirical hilltop foliage scatter factor for 1 obstruction  */
	sf2 = 1.0;		/* average empirical hilltop foliage scatter factor for 2 obstructions */

	/* dfdh=prop.dh; */
	/* ec=0.5*prop.gme; */

	/* adiff2 must first be run with d==0.0 to set up coefficients */
	if d == 0.0 {
		q = prop.hg[0] * prop.hg[1];
		qk = prop.he[0] * prop.he[1] - q;
		/* dhec=2.73; */

		if prop.mdp < 0 {
			q += 10.0;
        }

		/* coefficients for a standard four radii, rounded earth computation are prepared */
		wd1 = (1.0 + qk / q).sqrt();
		xd1 = propa.dla + propa.tha / prop.gme;
		q = (1.0 - 0.8 * (-propa.dlsa / 50e3).exp()) * prop.dh;
		q *= 0.78 * (-pow(q / 16.0, 0.25)).exp();
		qk = 1.0 / prop_zgnd.norm();
		aht = 20.0;
		xht = 0.0;
		a = 0.5 * (prop.dl[0] * prop.dl[0]) / prop.he[0];
		wa = pow(a * prop.wn, THIRD);
		pk = qk / wa;
		q = (1.607 - pk) * 151.0 * wa * prop.dl[0] / a;
		xht = q;
		aht += fht(q, pk);

		if prop.dl[1].floor() == 0.0 || (prop.the[1] > 0.2) {
			xht += xht;
			aht += aht - 20.0;
		}

		else {
			a = 0.5 * (prop.dl[1] * prop.dl[1]) / prop.he[1];
			wa = pow(a * prop.wn, THIRD);
			pk = qk / wa;
			q = (1.607 - pk) * 151.0 * wa * prop.dl[1] / a;
			xht += q;
			aht += fht(q, pk);
		}
		adiffv2 = 0.0;
	}

	else {
		th = propa.tha + d * prop.gme;

		dsl = mymax(d - propa.dla, 0.0);
		ds = d - propa.dla;
		a = ds / th;
		wa = pow(a * prop.wn, THIRD);
		pk = qk / wa;
		toh =
		    prop.hht - (prop.rch[0] -
				prop.dl[0] * ((prop.rch[1] - prop.rch[0]) /
					      prop.dist));
		roh =
		    prop.hhr - (prop.rch[0] -
				(prop.dist -
				 prop.dl[1]) * ((prop.rch[1] -
						 prop.rch[0]) / prop.dist));
		toho =
		    prop.hht - (prop.rch[0] -
				(prop.dl[0] +
				 dsl) * ((prop.hhr - prop.rch[0]) / (prop.dist -
								     prop.
								     dl[1])));
		roho =
		    prop.hhr - (prop.hht -
				dsl * ((prop.rch[1] - prop.hht) / dsl));
		dto = sqrt(prop.dl[0] * prop.dl[0] + toh * toh);
		dto += prop.gme * prop.dl[0];
		dto1 = sqrt(prop.dl[0] * prop.dl[0] + toho * toho);
		dto1 += prop.gme * prop.dl[0];
		dtro =
		    sqrt((prop.dl[0] + dsl) * (prop.dl[0] + dsl) +
			 prop.hhr * prop.hhr);
		dtro += prop.gme * (prop.dl[0] + dsl);
		drto =
		    sqrt((prop.dl[1] + dsl) * (prop.dl[1] + dsl) +
			 prop.hht * prop.hht);
		drto += prop.gme * (prop.dl[1] + dsl);
		dro = sqrt(prop.dl[1] * prop.dl[1] + roh * roh);
		dro += prop.gme * (prop.dl[1]);
		dro2 = sqrt(prop.dl[1] * prop.dl[1] + roho * roho);
		dro2 += prop.gme * (prop.dl[1]);
		dtr =
		    sqrt(prop.dist * prop.dist +
			 (prop.rch[0] - prop.rch[1]) * (prop.rch[0] -
							prop.rch[1]));
		dtr += prop.gme * prop.dist;
		dhh1 =
		    sqrt((prop.dist - propa.dla) * (prop.dist - propa.dla) +
			 toho * toho);
		dhh1 += prop.gme * (prop.dist - propa.dla);
		dhh2 =
		    sqrt((prop.dist - propa.dla) * (prop.dist - propa.dla) +
			 roho * roho);
		dhh2 += prop.gme * (prop.dist - propa.dla);

		/* for 1 obst tree base path */
		dtof =
		    sqrt(prop.dl[0] * prop.dl[0] +
			 (toh - prop.cch) * (toh - prop.cch));
		dtof += prop.gme * prop.dl[0];
		dto1f =
		    sqrt(prop.dl[0] * prop.dl[0] +
			 (toho - prop.cch) * (toho - prop.cch));
		dto1f += prop.gme * prop.dl[0];
		drof =
		    sqrt(prop.dl[1] * prop.dl[1] +
			 (roh - prop.cch) * (roh - prop.cch));
		drof += prop.gme * (prop.dl[1]);
		dro2f =
		    sqrt(prop.dl[1] * prop.dl[1] +
			 (roho - prop.cch) * (roho - prop.cch));
		dro2f += prop.gme * (prop.dl[1]);

		/* saalos coefficients preset for post-obstacle receive path */
		prop.tgh = prop.cch + 1.0;
		prop.tsgh = prop.hhr;
		rd = prop.dl[1];

		/* two obstacle diffraction calculation */
		if ds.floor() > 0.0 {	/* there are 2 obstacles */
			if prop.dl[1].floor() > 0.0 {	/* receive site past 2nd peak */
				/* rounding attenuation */
				q = (1.607 - pk) * 151.0 * wa * th + xht;
				/* ar=0.05751*q-10*log10(q)-aht; */

				/* knife edge vs round weighting */
				q = (1.0 - 0.8 * exp(-d / 50e3)) * prop.dh;
				q = (wd1 + xd1 / d) * f64::min(q * prop.wn,
							    6283.2);
				/* wd=25.1/(25.1+sqrt(q)); */

				q = 0.6365 * prop.wn;

				if prop.the[1] < 0.2 {	/* receive grazing angle below 0.2 rad */
					/* knife edge attenuation for two obstructions */

					if prop.hht < 3400.0 {	/* if below tree line, foliage top loss */
						vv = q * abs(dto1 + dhh1 -
							     dtro);
						adiffv2 =
						    -18.0 + sf2 * aknfe(vv);
					} else {
						vv = q * abs(dto1 + dhh1 -
							     dtro);
						adiffv2 = aknfe(vv);
					}

					if prop.hhr < 3400.0 {
						vv = q * abs(dro2 + dhh2 -
							     drto);
						adiffv2 +=
						    -18.0 + sf2 * aknfe(vv);
					} else {
						vv = q * abs(dro2 + dhh2 -
							     drto);
						adiffv2 += aknfe(vv);
					}
					/* finally, add clutter loss */
					closs = saalos(rd, prop, propa);
					adiffv2 += f64::min(22.0, closs);

				} else {	/* rcvr site too close to 2nd obs */

					/* knife edge attenuation for 1st obs */

					if prop.hht < 3400.0 {
						vv = q * abs(dto1 + dhh1 -
							     dtro);
						adiffv2 =
						    -18.0 + sf2 * aknfe(vv);
					} else {
						vv = q * abs(dto1 + dhh1 -
							     dtro);
						adiffv2 = aknfe(vv);
					}

					/* weighted calc. of knife vs rounded edge 
					   adiffv2=ar*wd+(1.0-wd)*adiffv2; */

					/* clutter path loss past 2nd peak */
					if prop.the[1] < 1.22 {
						rd = prop.dl[1];

						if prop.the[1] > 0.6 {	/* through foliage downhill */
							prop.tgh = prop.cch;
						} else {	/* close to foliage, rcvr in foliage downslope */

							vv = 0.6365 * prop.wn *
							    abs(dro2 + dhh2 -
								drto);
						}
						adiffv2 += aknfe(vv);
						closs = saalos(rd, prop, propa);
						adiffv2 += f64::min(closs, 22.0);
					} else {	/* rcvr very close to bare cliff or skyscraper */

						adiffv2 = 5.8 + 25.0;
					}
				}
			} else {	/* receive site is atop a 2nd peak */

				vv = 0.6365 * prop.wn * abs(dto + dro - dtr);
				adiffv2 = 5.8 + aknfe(vv);
			}
		} else {	/* for single obstacle */

			if prop.dl[1].floor() > 0.0 {	/* receive site past 1st peak */

				if prop.the[1] < 0.2 {	/* receive grazing angle less than .2 radians */
					vv = 0.6365 * prop.wn * abs(dto + dro -
								    dtr);

					if prop.hht < 3400.0 {
						sdl = 18.0;
						sdl = pow(10.0, -sdl / 20.0);
						/* ke phase difference with respect to direct t-r line */
						kedr =
						    0.159155 * prop.wn *
						    abs(dto + dro - dtr);
						arp = abs(kedr - kedr.floor());
						kem = aknfe(vv);
						kem = pow(10.0, -kem / 20.0);
						/* scatter path phase with respect to direct t-r line */
						sdr =
						    0.5 +
						    0.159155 * prop.wn *
						    abs(dtof + drof - dtr);
						srp = abs(sdr - sdr.floor());
						/* difference between scatter and ke phase in radians */
						pd = 6.283185307 * abs(srp -
								       arp);
						/* report pd prior to restriction 
						   keep pd between 0 and pi radians and adjust for 3&4 quadrant */
						if pd >= 3.141592654 {
							pd = 6.283185307 - pd;
							csd =
                            abq_alos(Complex::new(sdl, 0.0) + Complex::new(kem * -cos(pd), kem * -sin(pd)));
						} else {
							csd =
                            abq_alos(Complex::new(sdl, 0.0) + Complex::new(kem * cos(pd), kem * sin(pd)));
						}
						/*csd=mymax(csd,0.0009); limits maximum loss value to 30.45 db */
						adiffv2 =
						    -3.71 - 10.0 * log10(csd);
					} else {
						adiffv2 = aknfe(vv);
					}
					/* finally, add clutter loss */
					closs = saalos(rd, prop, propa);
					adiffv2 += f64::min(closs, 22.0);
				} else {	/* receive grazing angle too high */

					if prop.the[1] < 1.22 {
						rd = prop.dl[1];

						if prop.the[1] > 0.6 {	/* through foliage downhill */
							prop.tgh = prop.cch;
						} else {	/* downhill slope just above foliage  */

							vv = 0.6365 * prop.wn *
							    abs(dto + dro -
								dtr);
							adiffv2 = aknfe(vv);
						}
						closs = saalos(rd, prop, propa);
						adiffv2 += f64::min(22.0, closs);
					} else {	/* receiver very close to bare cliff or skyscraper */

						adiffv2 = 5.8 + 25.0;
					}
				}
			} else {	/* if occurs, receive site atop first peak  */

				adiffv2 = 5.8;
			}
		}
	}
	return adiffv2;
}

unsafe fn ascat(d: f64, prop: &prop_type, propa: &propa_type) -> f64
{
    static mut ad : f64 = 0.0; 
    static mut rr : f64 = 0.0; 
    static mut etq : f64 = 0.0; 
    static mut h0s : f64 = 0.0;
	//static thread_local double ad, rr, etq, h0s;
    let mut h0; let mut r1; let mut r2; let mut z0; let mut ss; let mut et; let mut ett; let mut th;
    let mut q; let mut ascatv; let mut temp;

	if d == 0.0 {
		ad = prop.dl[0] - prop.dl[1];
		rr = prop.he[1] / prop.rch[0];

		if ad < 0.0 {
			ad = -ad;
			rr = 1.0 / rr;
		}

		etq = (5.67e-6 * prop.ens - 2.32e-3) * prop.ens + 0.031;
		h0s = -15.0;
		ascatv = 0.0;
	}

	else {
		if h0s > 15.0 {
			h0 = h0s;
        } else {
			th = prop.the[0] + prop.the[1] + d * prop.gme;
			r2 = 2.0 * prop.wn * th;
			r1 = r2 * prop.he[0];
			r2 *= prop.he[1];

			if r1 < 0.2 && r2 < 0.2 {
				return 1001.0;	// <==== early return
            }

			ss = (d - ad) / (d + ad);
			q = rr / ss;
			ss = mymax(0.1, ss);
			q = f64::min(mymax(0.1, q), 10.0);
			z0 = (d - ad) * (d + ad) * th * 0.25 / d;
			/* et=(etq*exp(-pow(mymin(1.7,z0/8.0e3),6.0))+1.0)*z0/1.7556e3; */

			temp = f64::min(1.7, z0 / 8.0e3);
			temp = temp * temp * temp * temp * temp * temp;
			et = (etq * exp(-temp) + 1.0) * z0 / 1.7556e3;

			ett = mymax(et, 1.0);
			h0 = (h0f(r1, ett) + h0f(r2, ett)) * 0.5;
			h0 +=
            f64::min(h0,
				  (1.38 - log(ett)) * log(ss) * log(q) * 0.49);
			h0 = fortran_dim(h0, 0.0);

			if et < 1.0 {
				/* h0=et*h0+(1.0-et)*4.343*log(pow((1.0+1.4142/r1)*(1.0+1.4142/r2),2.0)*(r1+r2)/(r1+r2+2.8284)); */

				temp =
				    (1.0 + 1.4142 / r1) * (1.0 + 1.4142 / r2);
				h0 = et * h0 + (1.0 -
						et) * 4.343 * log((temp *
								   temp) * (r1 +
									    r2)
								  / (r1 + r2 +
								     2.8284));
			}

			if h0 > 15.0 && h0s >= 0.0 {
				h0 = h0s;
            }
		}

		h0s = h0;
		th = propa.tha + d * prop.gme;
		/* ascatv=ahd(th*d)+4.343*log(47.7*prop.wn*pow(th,4.0))-0.1*(prop.ens-301.0)*exp(-th*d/40e3)+h0; */
		ascatv =
		    ahd(th * d) +
		    4.343 * log(47.7 * prop.wn * (th * th * th * th)) -
		    0.1 * (prop.ens - 301.0) * exp(-th * d / 40e3) + h0;
	}

	return ascatv;
}

unsafe fn avar(zzt: f64, zzl: f64, zzc: f64, prop: &mut prop_type, propv: &mut propv_type) -> f64
{
    static mut kdv : i32 = 0;
    static mut dexa : f64 = 0.0; 
    static mut de : f64 = 0.0; 
    static mut vmd : f64 = 0.0; 
    static mut vs0 : f64 = 0.0; 
    static mut sgl : f64 = 0.0; 
    static mut sgtm : f64 = 0.0; 
    static mut sgtp : f64 = 0.0; 
    static mut sgtd : f64 = 0.0; 
    static mut tgtd : f64 = 0.0;
    static mut gm : f64 = 0.0; 
    static mut gp : f64 = 0.0; 
    static mut cv1 : f64 = 0.0; 
    static mut cv2 : f64 = 0.0; 
    static mut yv1 : f64 = 0.0; 
    static mut yv2 : f64 = 0.0; 
    static mut yv3 : f64 = 0.0; 
    static mut csm1 : f64 = 0.0; 
    static mut csm2 : f64 = 0.0; 
    static mut ysm1 : f64 = 0.0; 
    static mut ysm2 : f64 = 0.0;
    static mut ysm3 : f64 = 0.0; 
    static mut csp1 : f64 = 0.0;
    static mut csp2 : f64 = 0.0;
    static mut ysp1 : f64 = 0.0;
    static mut ysp2 : f64 = 0.0;
    static mut ysp3 : f64 = 0.0;
    static mut csd1 : f64 = 0.0;
    static mut zd : f64 = 0.0;
    static mut cfm1 : f64 = 0.0;
    static mut cfm2 : f64 = 0.0;
    static mut cfm3 : f64 = 0.0; 
    static mut cfp1 : f64 = 0.0; 
    static mut cfp2 : f64 = 0.0; 
    static mut cfp3 : f64 = 0.0;
/*static thread_local int kdv;
static thread_local double dexa, de, vmd, vs0, sgl, sgtm, sgtp, sgtd, tgtd,
    gm, gp, cv1, cv2, yv1, yv2, yv3, csm1, csm2, ysm1, ysm2,
    ysm3, csp1, csp2, ysp1, ysp2, ysp3, csd1, zd, cfm1, cfm2,
    cfm3, cfp1, cfp2, cfp3;*/

    let bv1 = [-9.67, -0.62, 1.26, -9.21, -0.62, -0.39, 3.15];
    let bv2 = [12.7, 9.19, 15.5, 9.05, 9.19, 2.86, 857.9];
    let xv1 = [144.9e3, 228.9e3, 262.6e3, 84.1e3, 228.9e3, 141.7e3, 2222.0e3];
    let xv2 = [90.3e3, 205.2e3, 185.2e3, 101.1e3, 205.2e3, 315.9e3, 164.8e3];
    let xv3 = [133.8e3, 143.6e3, 99.8e3, 98.6e3, 143.6e3, 167.4e3, 116.3e3];
    let bsm1 = [2.13, 2.66, 6.11, 1.98, 2.68, 6.86, 8.51];
    let bsm2 = [159.5, 7.67, 6.65, 13.11, 7.16, 10.38, 169.8];
    let xsm1 = [762.2e3, 100.4e3, 138.2e3, 139.1e3, 93.7e3, 187.8e3, 609.8e3];
    let xsm2 = [123.6e3, 172.5e3, 242.2e3, 132.7e3, 186.8e3, 169.6e3, 119.9e3];
    let xsm3 = [94.5e3, 136.4e3, 178.6e3, 193.5e3, 133.5e3, 108.9e3, 106.6e3];
    let bsp1 = [2.11, 6.87, 10.08, 3.68, 4.75, 8.58, 8.43];
    let bsp2 = [102.3, 15.53, 9.60, 159.3, 8.12, 13.97, 8.19];
    let xsp1 = [636.9e3, 138.7e3, 165.3e3, 464.4e3, 93.2e3, 216.0e3, 136.2e3];
    let xsp2 = [134.8e3, 143.7e3, 225.7e3, 93.1e3, 135.9e3, 152.0e3, 188.5e3];
    let xsp3 = [95.6e3, 98.6e3, 129.7e3, 94.2e3, 113.4e3, 122.7e3, 122.9e3];
    let bsd1 = [1.224, 0.801, 1.380, 1.000, 1.224, 1.518, 1.518];
    let bzd1 = [1.282, 2.161, 1.282, 20., 1.282, 1.282, 1.282];
    let bfm1 = [1.0, 1.0, 1.0, 1.0, 0.92, 1.0, 1.0];
    let bfm2 = [0.0, 0.0, 0.0, 0.0, 0.25, 0.0, 0.0];
    let bfm3 = [0.0, 0.0, 0.0, 0.0, 1.77, 0.0, 0.0];
    let bfp1 = [ 1.0, 0.93, 1.0, 0.93, 0.93, 1.0, 1.0 ];
    let bfp2 = [ 0.0, 0.31, 0.0, 0.19, 0.31, 0.0, 0.0 ];
    let bfp3 = [ 0.0, 2.00, 0.0, 1.79, 2.00, 0.0, 0.0 ];


//double bv1[7] = { -9.67, -0.62, 1.26, -9.21, -0.62, -0.39, 3.15 };
//double bv2[7] = { 12.7, 9.19, 15.5, 9.05, 9.19, 2.86, 857.9 };
//double xv1[7] =
//    { 144.9e3, 228.9e3, 262.6e3, 84.1e3, 228.9e3, 141.7e3, 2222.e3 };
//double xv2[7] =
//    { 190.3e3, 205.2e3, 185.2e3, 101.1e3, 205.2e3, 315.9e3, 164.8e3 };
//double xv3[7] =
//    { 133.8e3, 143.6e3, 99.8e3, 98.6e3, 143.6e3, 167.4e3, 116.3e3 };
//double bsm1[7] = { 2.13, 2.66, 6.11, 1.98, 2.68, 6.86, 8.51 };
//double bsm2[7] = { 159.5, 7.67, 6.65, 13.11, 7.16, 10.38, 169.8 };
//double xsm1[7] =
//    { 762.2e3, 100.4e3, 138.2e3, 139.1e3, 93.7e3, 187.8e3, 609.8e3 };
//double xsm2[7] =
//    { 123.6e3, 172.5e3, 242.2e3, 132.7e3, 186.8e3, 169.6e3, 119.9e3 };
//double xsm3[7] =
//    { 94.5e3, 136.4e3, 178.6e3, 193.5e3, 133.5e3, 108.9e3, 106.6e3 };
//double bsp1[7] = { 2.11, 6.87, 10.08, 3.68, 4.75, 8.58, 8.43 };
//double bsp2[7] = { 102.3, 15.53, 9.60, 159.3, 8.12, 13.97, 8.19 };
//double xsp1[7] =
//    { 636.9e3, 138.7e3, 165.3e3, 464.4e3, 93.2e3, 216.0e3, 136.2e3 };
//double xsp2[7] =
//    { 134.8e3, 143.7e3, 225.7e3, 93.1e3, 135.9e3, 152.0e3, 188.5e3 };
//double xsp3[7] =
//    { 95.6e3, 98.6e3, 129.7e3, 94.2e3, 113.4e3, 122.7e3, 122.9e3 };
//double bsd1[7] = { 1.224, 0.801, 1.380, 1.000, 1.224, 1.518, 1.518 };
//double bzd1[7] = { 1.282, 2.161, 1.282, 20., 1.282, 1.282, 1.282 };
//double bfm1[7] = { 1.0, 1.0, 1.0, 1.0, 0.92, 1.0, 1.0 };
//double bfm2[7] = { 0.0, 0.0, 0.0, 0.0, 0.25, 0.0, 0.0 };
//double bfm3[7] = { 0.0, 0.0, 0.0, 0.0, 1.77, 0.0, 0.0 };
//double bfp1[7] = { 1.0, 0.93, 1.0, 0.93, 0.93, 1.0, 1.0 };
//double bfp2[7] = { 0.0, 0.31, 0.0, 0.19, 0.31, 0.0, 0.0 };
//double bfp3[7] = { 0.0, 2.00, 0.0, 1.79, 2.00, 0.0, 0.0 };

    static mut ws : bool = false; 
    static mut w1 : bool = false;
//static thread_local bool ws, w1;
    let mut rt = 7.8;
    let mut rl = 24.0;
    let mut avarv; let mut q; let mut vs; let mut zt; let mut zl; let mut zc;
    let mut sgt; let mut yr; let mut temp1; let mut temp2;
    let mut temp_klim = propv.klim - 1;

    if propv.lvar > 0 {
        match propv.lvar {
        //switch (propv.lvar) {
        _ => {
            if propv.klim <= 0 || propv.klim > 7 {
                propv.klim = 5;
                temp_klim = 4;
                prop.kwx = i32::max(prop.kwx, 2);
            }

            cv1 = bv1[temp_klim as usize];
            cv2 = bv2[temp_klim as usize];
            yv1 = xv1[temp_klim as usize];
            yv2 = xv2[temp_klim as usize];
            yv3 = xv3[temp_klim as usize];
            csm1 = bsm1[temp_klim as usize];
            csm2 = bsm2[temp_klim as usize];
            ysm1 = xsm1[temp_klim as usize];
            ysm2 = xsm2[temp_klim as usize];
            ysm3 = xsm3[temp_klim as usize];
            csp1 = bsp1[temp_klim as usize];
            csp2 = bsp2[temp_klim as usize];
            ysp1 = xsp1[temp_klim as usize];
            ysp2 = xsp2[temp_klim as usize];
            ysp3 = xsp3[temp_klim as usize];
            csd1 = bsd1[temp_klim as usize];
            zd = bzd1[temp_klim as usize];
            cfm1 = bfm1[temp_klim as usize];
            cfm2 = bfm2[temp_klim as usize];
            cfm3 = bfm3[temp_klim as usize];
            cfp1 = bfp1[temp_klim as usize];
            cfp2 = bfp2[temp_klim as usize];
            cfp3 = bfp3[temp_klim as usize];
        }

        4 => {
        //case 4:
            kdv = propv.mdvar;
            ws = kdv >= 20;

            if ws {
                kdv -= 20;
            }

            w1 = kdv >= 10;

            if w1 {
                kdv -= 10;
            }

            if kdv < 0 || kdv > 3 {
                kdv = 0;
                prop.kwx = i32::max(prop.kwx, 2);
            }
        }

        3 => {
            q = log(0.133 * prop.wn);

            /* gm=cfm1+cfm2/(pow(cfm3*q,2.0)+1.0); */
            /* gp=cfp1+cfp2/(pow(cfp3*q,2.0)+1.0); */

            gm = cfm1 + cfm2 / ((cfm3 * q * cfm3 * q) + 1.0);
            gp = cfp1 + cfp2 / ((cfp3 * q * cfp3 * q) + 1.0);
        }

        2 => {
            dexa =
                sqrt(18e6 * prop.he[0]) + sqrt(18e6 * prop.he[1]) +
                pow(575.7e12 / prop.wn, THIRD);
        }

        1 => {
            if prop.dist < dexa {
                de = 130e3 * prop.dist / dexa;
            } else {
                de = 130e3 + prop.dist - dexa;
            }
        }
        } // End match
        // NOTE: Warning - this switch didn't have break, so it may need to handle falling through?

        vmd = curve(cv1, cv2, yv1, yv2, yv3, de);
        sgtm = curve(csm1, csm2, ysm1, ysm2, ysm3, de) * gm;
        sgtp = curve(csp1, csp2, ysp1, ysp2, ysp3, de) * gp;
        sgtd = sgtp * csd1;
        tgtd = (sgtp - sgtd) * zd;

        if w1 {
            sgl = 0.0;
        }
        else {
            q = (1.0 -
                0.8 * exp(-prop.dist / 50e3)) * prop.dh * prop.wn;
            sgl = 10.0 * q / (q + 13.0);
        }

        if ws {
            vs0 = 0.0;
        }
        else {
            /* vs0=pow(5.0+3.0*exp(-de/100e3),2.0); */
            temp1 = 5.0 + 3.0 * exp(-de / 100e3);
            vs0 = temp1 * temp1;

        }

        propv.lvar = 0;
    }

    zt = zzt;
    zl = zzl;
    zc = zzc;

    match kdv {
        0 => {
            zt = zc;
            zl = zc;
        } // This one had a break

        1 => {
            zl = zc;
        } // Likewise

        2 => {
            zl = zt;
        }

        _ => {}
    }

    if fabs(zt) > 3.1 || fabs(zl) > 3.1 || fabs(zc) > 3.1 {
        prop.kwx = i32::max(prop.kwx, 1);
    }

    if zt < 0.0 {
        sgt = sgtm;
    }
    else if zt <= zd {
        sgt = sgtp;
    }
    else {
        sgt = sgtd + tgtd / zt;
    }

    /* vs=vs0+pow(sgt*zt,2.0)/(rt+zc*zc)+pow(sgl*zl,2.0)/(rl+zc*zc); */

    temp1 = sgt * zt;
    temp2 = sgl * zl;

    vs = vs0 + (temp1 * temp1) / (rt + zc * zc) + (temp2 * temp2) / (rl +
                                    zc *
                                    zc);

    if kdv == 0 {
        yr = 0.0;
        propv.sgc = sqrt(sgt * sgt + sgl * sgl + vs);
    }

    else if kdv == 1 {
        yr = sgt * zt;
        propv.sgc = sqrt(sgl * sgl + vs);
    }

    else if kdv == 2 {
        yr = sqrt(sgt * sgt + sgl * sgl) * zt;
        propv.sgc = sqrt(vs);
    }

    else {
        yr = sgt * zt + sgl * zl;
        propv.sgc = sqrt(vs);
    }

    avarv = prop.aref - vmd - yr - propv.sgc * zc;

    if avarv < 0.0 {
        avarv = avarv * (29.0 - avarv) / (29.0 - 10.0 * avarv);
    }

    return avarv;
}

#[cfg(test)]
mod test {
	use crate::{PTPPath, PTPClimate};
	use float_cmp::approx_eq;
	use super::point_to_point;

	#[test]
    fn basic_fspl_test() {
        let terrain_path = PTPPath::new(
			vec![1.0; 200], 
			100.0, 
			100.0, 
			10.0
		).unwrap();

		let climate = PTPClimate::default();

		let mut dbloss = 0.0;
		let mut strmode = String::new();
		let mut errnum = 0;

		point_to_point(
			&terrain_path.elevations,
			terrain_path.transmit_height,
			terrain_path.receive_height,
			climate.eps_dialect,
			climate.sgm_conductivity,
			climate.eno_ns_surfref,
			5800.0,
			climate.radio_climate,
			1,
			0.5,
			0.5,
			&mut dbloss,
			&mut strmode,
			&mut errnum
		);

		println!("Mode: {}, Loss: {}, Error: {}", strmode, dbloss, errnum );

        assert_eq!(strmode, "L-o-S");
        assert_eq!(errnum, 0);
		let closs = format!("{:.2}", dbloss);
        assert_eq!(closs, "113.65"); // Checking to 2 decimal places. Don't need more than that.
    }

	#[test]
    fn basic_one_obstruction() {
        let mut elevations = vec![1.0; 200];
        elevations[100] = 110.0;
        let mut terrain_path = PTPPath::new(elevations, 100.0, 100.0, 10.0).unwrap();
		let climate = PTPClimate::default();

        let mut dbloss = 0.0;
		let mut strmode = String::new();
		let mut errnum = 0;

		point_to_point(
			&terrain_path.elevations,
			terrain_path.transmit_height,
			terrain_path.receive_height,
			climate.eps_dialect,
			climate.sgm_conductivity,
			climate.eno_ns_surfref,
			5800.0,
			climate.radio_climate,
			1,
			0.5,
			0.5,
			&mut dbloss,
			&mut strmode,
			&mut errnum
		);

		println!("Mode: {}, Loss: {}, Error: {}", strmode, dbloss, errnum );

        assert_eq!(strmode, "1_Hrzn_Diff");
        assert_eq!(errnum, 0);
    }

    #[test]
    fn basic_two_obstructions() {
        let mut elevations = vec![1.0; 200];
        elevations[100] = 110.0;
        elevations[150] = 110.0;
        let mut terrain_path = PTPPath::new(elevations, 100.0, 100.0, 10.0).unwrap();
		let climate = PTPClimate::default();

        let mut dbloss = 0.0;
		let mut strmode = String::new();
		let mut errnum = 0;

		point_to_point(
			&terrain_path.elevations,
			terrain_path.transmit_height,
			terrain_path.receive_height,
			climate.eps_dialect,
			climate.sgm_conductivity,
			climate.eno_ns_surfref,
			5800.0,
			climate.radio_climate,
			1,
			0.5,
			0.5,
			&mut dbloss,
			&mut strmode,
			&mut errnum
		);

		println!("Mode: {}, Loss: {}, Error: {}", strmode, dbloss, errnum );

        assert_eq!(strmode, "2_Hrzn_Diff");
        assert_eq!(errnum, 0);
    }
}