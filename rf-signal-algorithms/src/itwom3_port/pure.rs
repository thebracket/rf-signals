// Functions in this module are "pure" - they don't mutate state.
// Separating these out to help find the thread_local/static C that
// needs to retain state.

use super::helpers::*;
use super::{prop_type, propa_type};
use num_complex::Complex;

pub(crate) fn curve(c1: f64, c2: f64, x1: f64, x2: f64, x3: f64, de: f64) -> f64
{
    let mut temp1 = (de - x2) / x3;
    let mut temp2 = de / x1;

    temp1 *= temp1;
    temp2 *= temp2;

    (c1 + c2 / (1.0 + temp1)) * temp2 / (1.0 + temp2)
}

pub(crate) fn ahd(td: f64) -> f64
{
    let i;
	//double a[3] = { 133.4, 104.6, 71.8 };
	//double b[3] = { 0.332e-3, 0.212e-3, 0.157e-3 };
	//double c[3] = { -4.343, -1.086, 2.171 };
    let a = [133.4, 104.6, 71.8];
    let b = [0.332e-3, 0.212e-3, 0.157e-3];
    let c = [-4.343, -1.086, 2.171];

	if td <= 10e3 {
		i = 0;
    }
	else if td <= 70e3 {
		i = 1;
    }
	else {
		i = 2;
    }

	return a[i] + b[i] * td + c[i] * td.ln();
}

pub(crate) fn h0f(r: f64, et: f64) -> f64
{
    let a = [25.0, 80.0, 177.0, 395.0, 705.0];
    let b = [24.0, 45.0, 68.0, 80.0, 105.0];
	//double a[5] = { 25.0, 80.0, 177.0, 395.0, 705.0 };
	//double b[5] = { 24.0, 45.0, 68.0, 80.0, 105.0 };
    let q; let x; let mut h0fv; let temp; let mut it;

	it = et as i32;

	if it <= 0 {
		it = 1;
		q = 0.0;
	}

	else if it >= 5 {
		it = 5;
		q = 0.0;
	}

	else {
		q = et - it as f64;
    }

	/* x=pow(1.0/r,2.0); */

	temp = 1.0 / r;
	x = temp * temp;

	h0fv = 4.343 * log((a[it as usize - 1] * x + b[it as usize - 1]) * x + 1.0);

	if q != 0.0 {
		h0fv =
		    (1.0 - q) * h0fv + q * 4.343 * log((a[it as usize] * x + b[it as usize]) * x +
						       1.0);
    }

	return h0fv;
}

pub(crate) fn saalos(d: f64, prop: &prop_type, propa: &propa_type) -> f64
{
    let mut ensa;
    let mut encca;
    let mut q;
    let mut dp;
    let mut dx;
    let mut tde;
    let mut ucrpc;
    let mut ctip;
    let mut tip;
    let mut tic = 0.0;
    let mut stic;
    let mut ctic = 0.0;
    let mut sta;
    let mut ttc;
    let mut crpc = 0.0;
    let mut ssnps = 0.0;
    let mut d1a;
    let mut rsp;
    let mut tsp;
    let mut arte;
    let mut zi;
    let mut pd;
    let mut pdk;
    let mut hone;
    let mut tvsr;
    let mut saalosv = 0.0;
    let mut hc;
    let mut cttc = 0.0;

	q = 0.0;

	if d == 0.0 {
		tsp = 1.0;
		rsp = 0.0;
		d1a = 50.0;
		saalosv = 0.0;
	} else if prop.hg[1] > prop.cch {
		saalosv = 0.0;
	} else {
		pd = d;
		pdk = pd / 1000.0;
		tsp = 1.0;
		rsp = 0.0;
		d1a = pd;
		/* at first, hone is transmitter antenna height 
		   relative to receive site ground level. */
		hone = prop.tgh + prop.tsgh - (prop.rch[1] - prop.hg[1]);

		if prop.tgh > prop.cch {	/* for TX ant above all clutter height */
			ensa = 1.0 + prop.ens * 0.000001;
			encca = 1.0 + prop.encc * 0.000001;
			dp = pd;

            for j in 0..5 {
				tde = dp / 6378137.0;
				hc = (prop.cch + 6378137.0) * (1.0 - cos(tde));
				dx = (prop.cch + 6378137.0) * sin(tde);
				ucrpc =
				    sqrt((hone - prop.cch + hc) * (hone -
								   prop.cch +
								   hc) +
					 (dx * dx));
				ctip = (hone - prop.cch + hc) / ucrpc;
				tip = acos(ctip);
				tic = tip + tde;
				tic = f64::max(0.0, tic);
				stic = sin(tic);
				sta = (ensa / encca) * stic;
				ttc = asin(sta);
				cttc = sqrt(1.0 - (sin(ttc)) * (sin(ttc)));
				crpc = (prop.cch - prop.hg[1]) / cttc;
				if crpc >= dp {
					crpc = dp - 1.0 / dp;
				}

				ssnps = (3.1415926535897 / 2.0) - tic;
				d1a = (crpc * sin(ttc)) / (1.0 - 1.0 / 6378137.0);
				dp = pd - d1a;

			}

			ctic = cos(tic);

			/* if the ucrpc path touches the canopy before reaching the
			   end of the ucrpc, the entry point moves toward the
			   transmitter, extending the crpc and d1a. Estimating the d1a: */

			if ssnps <= 0.0 {
				d1a = f64::min(0.1 * pd, 600.0);
				crpc = d1a;
				/* hone must be redefined as being barely above
				   the canopy height with respect to the receiver
				   canopy height, which despite the earth curvature
				   is at or above the transmitter antenna height. */
				hone = prop.cch + 1.0;
				rsp = 0.997;
				tsp = 1.0 - rsp;
			} else {

				if prop.ptx >= 1 {	/* polarity ptx is vertical or circular */
					q = (ensa * cttc -
					      encca * ctic) / (ensa * cttc +
							       encca * ctic);
					rsp = q * q;
					tsp = 1.0 - rsp;

					if prop.ptx == 2 {	/* polarity is circular - new */
						q = (ensa * ctic -
						      encca * cttc) / (ensa *
								       ctic +
								       encca *
								       cttc);
						rsp =
						    (ensa * cttc -
						      encca * ctic) / (ensa *
								       cttc +
								       encca *
								       ctic);
						rsp = (q * q + rsp * rsp) / 2.0;
						tsp = 1.0 - rsp;
					}
				} else {	/* ptx is 0, horizontal, or undefined */

					q = (ensa * ctic -
					      encca * cttc) / (ensa * ctic +
							       encca * cttc);
					rsp = q * q;
					tsp = 1.0 - rsp;
				}
			}
			/* tvsr is defined as tx ant height above receiver ant height */
			tvsr = f64::max(0.0, prop.tgh + prop.tsgh - prop.rch[1]);

			if d1a < 50.0 {
				arte = 0.0195 * crpc - 20.0 * log10(tsp);
			}

			else {
				if d1a < 225.0 {

					if tvsr > 1000.0 {
						q = d1a * (0.03 *
							   exp(-0.14 * pdk));
					} else {
						q = d1a * (0.07 *
							   exp(-0.17 * pdk));
					}

					arte =
					    q + (0.7 * pdk -
						 f64::max(0.01,
						       log10(prop.wn * 47.7) -
						       2.0)) * (prop.hg[1] /
							      hone);
				}

				else {
					q = 0.00055 * (pdk) +
					    log10(pdk) * (0.041 -
							  0.0017 * sqrt(hone) +
							  0.019);

					arte =
					    d1a * q -
					    (18.0 * log10(rsp)) /
					    (exp(hone / 37.5));

					zi = 1.5 * sqrt(hone - prop.cch);

					if pdk > zi {
						q = (pdk -
						     zi) * 10.2 *
						    ((sqrt
						      (f64::max
						       (0.01,
							log10(prop.wn * 47.7) -
							2.0))) / (100.0 - zi));
					} else {
						q = ((zi -
						      pdk) / zi) * (-20.0 *
								    f64::max(0.01,
									  log10
									  (prop.
									   wn *
									   47.7)
									  -
									  2.0))
						    / sqrt(hone);
					}
					arte = arte + q;

				}
			}
		} else {	/* for TX at or below clutter height */

			q = (prop.cch - prop.tgh) * (2.06943 -
						     1.56184 * exp(1.0 /
								   prop.cch -
								   prop.tgh));
			q = q + (17.98 -
				 0.84224 * (prop.cch -
					    prop.tgh)) * exp(-0.00000061 * pd);
			arte = q + 1.34795 * 20.0 * log10(pd + 1.0);
			arte =
			    arte -
			    (f64::max(0.01, log10(prop.wn * 47.7) - 2.0)) *
			    (prop.hg[1] / prop.tgh);
		}
		saalosv = arte;
	}
	return saalosv;
}

pub(crate) fn qerfi(q: f64) -> f64
{
    let mut x;
    let mut t;
    let mut v;
	let c0 = 2.515516698;
	let c1 = 0.802853;
	let c2 = 0.010328;
	let d1 = 1.432788;
	let d2 = 0.189269;
	let d3 = 0.001308;

	x = 0.5 - q;
	t = mymax(0.5 - x.abs(), 0.000001);
	t = (-2.0 * t.ln()).sqrt();
	v = t - ((c2 * t + c1) * t + c0) / (((d3 * t + d2) * t + d1) * t + 1.0);

	if x < 0.0 {
		v = -v;
    }

	return v;
}

pub(crate) fn qlrps(fmhz: f64, zsys: f64, en0: f64, ipol: i32, eps: f64, sgm: f64, prop: &mut prop_type)
{
    let gma = 157e-9;

    prop.wn = fmhz / 47.7;
    prop.ens = en0;

    if zsys != 0.0 {
        prop.ens *= (-zsys / 9460.0).exp();
    }

    prop.gme = gma * (1.0 - 0.04665 * (prop.ens / 179.3).exp());
    let mut prop_zgnd = Complex::new(prop.zgndreal, prop.zgndimag);
    let mut zq = Complex::new(eps, 376.62 * sgm / prop.wn);

    prop_zgnd = (zq - 1.0).sqrt();

    if ipol != 0 {
        prop_zgnd = prop_zgnd / zq;
    }

    prop.zgndreal = prop_zgnd.re;
    prop.zgndimag = prop_zgnd.im;
}

pub(crate) fn hzns2(pfl: &[f64], prop: &mut prop_type, propa: &propa_type)
{
    let mut wq;
    let mut np;
    let mut rp;
    //let mut i;
    //let mut j;
    let mut xi;
    let mut za;
    let mut zb;
    let mut qc;
    let mut q;
    let mut sb;
    let mut sa;
    let mut dr;
    let mut dshh;

	np = pfl[0] as usize; // Is this really a floor?
	xi = pfl[1];
	za = pfl[2] + prop.hg[0];
	zb = pfl[np + 2] + prop.hg[1];
	prop.tiw = xi;
	prop.ght = za;
	prop.ghr = zb;
	qc = 0.5 * prop.gme;
	q = qc * prop.dist;
	prop.the[1] = ((zb - za) / prop.dist).atan();
	prop.the[0] = (prop.the[1]) - q;
	prop.the[1] = -prop.the[1] - q;
	prop.dl[0] = prop.dist;
	prop.dl[1] = prop.dist;
	prop.hht = 0.0;
	prop.hhr = 0.0;
	prop.los = 1;

	if np >= 2 {
		sa = 0.0;
		sb = prop.dist;
		wq = true;

        for j in 1..np {
			sa += xi;
			q = pfl[j + 2] - (qc * sa + prop.the[0]) * sa - za;

			if q > 0.0 {
				prop.los = 0;
				prop.the[0] += q / sa;
				prop.dl[0] = sa;
				prop.the[0] = f64::min(prop.the[0], 1.569);
				prop.hht = pfl[j + 2];
				wq = false;
			}
		}

		if !wq {
            for i in 1..np {
				sb -= xi;
				q = pfl[np + 2 - i] - (qc * (prop.dist - sb) +
						       prop.the[1]) *
				    (prop.dist - sb) - zb;
				if q > 0.0 {
					prop.the[1] += q / (prop.dist - sb);
					prop.the[1] = f64::min(prop.the[1], 1.57);
					prop.the[1] =
					    mymax(prop.the[1], -1.568);
					prop.hhr = pfl[np + 2 - i];
					prop.dl[1] = mymax(0.0, prop.dist - sb);
				}
			}
			prop.the[0] =
			    ((prop.hht - za) / prop.dl[0]).atan() -
			    0.5 * prop.gme * prop.dl[0];
			prop.the[1] =
			    ((prop.hhr - zb) / prop.dl[1]).atan() -
			    0.5 * prop.gme * prop.dl[1];
		}
	}

	if (prop.dl[1]) < (prop.dist) {
		dshh = prop.dist - prop.dl[0] - prop.dl[1];

		if dshh as i32 == 0 {	/* one obstacle */
			dr = prop.dl[1] / (1.0 + zb / prop.hht);
		} else {	/* two obstacles */

			dr = prop.dl[1] / (1.0 + zb / prop.hhr);
		}
	} else {		/* line of sight  */

		dr = (prop.dist) / (1.0 + zb / za);
	}
	rp = 2 + ((0.5 + dr / xi).floor()) as i32;
	prop.rpl = rp;
	prop.rph = pfl[rp as usize];
}

pub(crate) fn z1sq2(z: &[f64], x1: f64, x2: f64, z0: &mut f64, zn: &mut f64)
{
    /* corrected for use with ITWOM */
    let mut xn;
    let mut xa;
    let mut xb;
    let mut x;
    let mut a;
    let mut b;
    let mut bn;
    let mut n;
    let mut ja;
    let mut jb;

    xn = z[0];
    xa = (FORTRAN_DIM(x1 / z[1], 0.0)).floor();
    xb = xn - (FORTRAN_DIM(xn, x2 / z[1])).floor();

    if xb <= xa {
        xa = FORTRAN_DIM(xa, 1.0);
        xb = xn - FORTRAN_DIM(xn, xb + 1.0);
    }

    ja = xa as i32;
    jb = xb as i32;
    xa = (2.0 * ((xb - xa) / 2.0))-1.0; // Note that there were some whacky type conversions here
    x = -0.5 * (xa + 1.0);
    xb += x;
    ja = jb - 1 - xa as i32;
    n = jb - ja;
    a = z[ja as usize + 2] + z[jb as usize + 2];
    b = (z[ja as usize + 2] - z[jb as usize + 2]) * x;
    bn = 2.0 * (x * x);

    for i in 2..n {
        ja += 1;
        x += 1.0;
        bn += x * x;
        a += z[ja as usize + 2];
        b += z[ja as usize + 2] * x;
    }

    a /= xa + 2.0;
    b = b / bn;
    *z0 = a - (b * xb);
    *zn = a + (b * (xn - xb));
}

pub(crate) fn d1thx2(pfl: &[f64], x1: f64, x2: f64, propa: &propa_type) -> f64
{
    let mut np;
    let mut ka;
    let mut kb;
    let mut n;
    let mut k;
    let mut kmx;
    let mut d1thx2v;
    let mut sn;
    let mut xa;
    let mut xb;
    let mut xc;

    //double *s;

    np = pfl[0] as i32; // Is this really just a floor?
    xa = x1 / pfl[1];
    xb = x2 / pfl[1];
    d1thx2v = 0.0;

    if xb - xa < 2.0 {
        // exit out
        return d1thx2v;
    }

    ka = (0.1 * (xb - xa + 8.0)) as i32;
    kmx = i32::max(25, (83350 / (pfl[1]) as i32) as i32);
    ka = i32::min(i32::max(4, ka), kmx);
    n = 10 * ka - 5;
    kb = n - ka + 1;
    sn = n - 1;
    let mut s = Vec::<f64>::with_capacity(n as usize +2);
    for i in 0..(n+2) { s.push(0.0); }
    s[0] = sn as f64;
    s[1] = 1.0;
    xb = (xb - xa) / sn as f64;
    k = (xa + 1.0) as i32;
    xc = xa - (k as f64);

    for j in 0..n {
        while xc > 0.0 && k < np {
            xc -= 1.0;
            k += 1;
        }

        s[j as usize + 2] = pfl[k as usize + 2] + (pfl[k as usize + 2] - pfl[k as usize + 1]) * xc;
        xc = xc + xb;
    }

    z1sq2(&s, 0.0, sn as f64, &mut xa, &mut xb);
    xb = (xb - xa) / sn as f64;

    for j in 0..n {
        s[j as usize + 2] -= xa;
        xa = xa + xb;
    }

    d1thx2v = qtile(n - 1, &mut s, ka - 1) - qtile(n - 1, &mut s, kb - 1); // Warning: double-check that i interpreted s+2 right
    d1thx2v /= 1.0 - 0.8 * (-(x2 - x1) / 50.0e3).exp();
    return d1thx2v;
}

pub(crate) fn qtile(nn: i32, a: &mut [f64], ir: i32) -> f64
{
    let mut q = 0.0;
    let mut r;
    let mut m;
    let mut n;
    let mut i = 0;
    let mut j;
    let mut j1 = 0;
    let mut i0 = 0;
    let mut k;
    let mut done = false;
    let mut goto10 = true;

	m = 0;
	n = nn;
	k = f64::min(f64::max(0.0, ir as f64), n as f64);

	while !done {
		if goto10 {
			q = a[k as usize + 2];
			i0 = m;
			j1 = n;
		}

		i = i0;

		while i <= n && a[i as usize + 2] >= q {
			i+=1;
        }

		if i > n {
			i = n;
        }

		j = j1;

		while j >= m && a[j as usize + 2] <= q {
			j-=1;
        }

		if j < m {
			j = m;
        }

		if i < j {
			r = a[i as usize];
			a[i as usize + 2] = a[j as usize + 2];
			a[j as usize + 2] = r;
			i0 = i + 1;
			j1 = j - 1;
			goto10 = false;
		}

		else if i < k as i32 {
			a[k as usize + 2] = a[i as usize + 2];
			a[i as usize + 2] = q;
			m = i + 1;
			goto10 = true;
		}

		else if j > k as i32 {
			a[k as usize + 2] = a[j as usize + 2];
			a[j as usize + 2] = q;
			n = j - 1;
			goto10 = true;
		}

		else {
			done = true;
        }
	}

	return q;
}

pub(crate) fn fht(x: f64, pk: f64) -> f64
{
    let mut w;
    let mut fhtv;

	if x < 200.0 {
		w = -pk.ln();

		if pk < 1.0e-5 || x * w * w * w > 5495.0 {
			fhtv = -117.0;

			if x > 1.0 {
				fhtv = 40.0 * x.log10() + fhtv;
            }
		} else {
			fhtv = 2.5e-5 * x * x / pk - 8.686 * w - 15.0;
        }
	}

	else {
		fhtv = 0.05751 * x - 10.0 * x.log10();

		if x < 2000.0 {
			w = 0.0134 * x * (-0.005 * x).exp();
			fhtv = (1.0 - w) * fhtv + w * (40.0 * x.log10() - 117.0);
		}
	}
	return fhtv;
}

pub(crate) fn alos2(d: f64, prop: &mut prop_type, propa: &propa_type) -> f64
{
    let mut prop_zgnd = Complex::new(prop.zgndreal, prop.zgndimag);
    let mut r = Complex::new(0.0, 0.0);
    let mut cd;
    let mut cr;
    let mut dr;
    let mut hr;
    let mut hrg;
    let mut ht;
    let mut htg;
    let mut hrp;
    let mut re;
    let mut s;
    let mut sps;
    let mut q;
    let mut pd;
    let mut drh;
	/* int rp; */
	let mut alosv;

	cd = 0.0;
	cr = 0.0;
	htg = prop.hg[0];
	hrg = prop.hg[1];
	ht = prop.ght;
	hr = prop.ghr;
	/* rp=prop.rpl; */
	hrp = prop.rph;
	pd = prop.dist;

	if d == 0.0 {
		alosv = 0.0;
	}

	else {
		q = prop.he[0] + prop.he[1];
		sps = q / (pd * pd + q * q).sqrt();
		q = (1.0 - 0.8 * (-pd / 50e3).exp()) * prop.dh;

		if prop.mdp < 0 {
			dr = pd / (1.0 + hrg / htg);

			if dr < (0.5 * pd) {
				drh =
				    6378137.0 - (-(0.5 * pd) * (0.5 * pd) +
						     6378137.0 * 6378137.0 +
						     (0.5 * pd -
						      dr) * (0.5 * pd - dr)).sqrt();
			} else {
				drh =
				    6378137.0 - (-(0.5 * pd) * (0.5 * pd) +
						     6378137.0 * 6378137.0 +
						     (dr - 0.5 * pd) * (dr -
									0.5 *
									pd)).sqrt();
			}

			if (sps < 0.05) && (prop.cch > hrg) && (prop.dist < prop.dl[0]) {	/* if far from transmitter and receiver below canopy */
				cd = mymax(0.01,
					   pd * (prop.cch - hrg) / (htg - hrg));
				cr = mymax(0.01,
					   pd - dr + dr * (prop.cch -
							   drh) / htg);
				q = (1.0 -
				      0.8 * (-pd / 50e3).exp()) * prop.dh *
				     (f64::min(-20.0 * (cd / cr).log10(), 1.0));
			}
		}

		s = 0.78 * q * (-pow(q / 16.0, 0.25)).exp();
		q = (-f64::min(10.0, prop.wn * s * sps)).exp();
		r = q * (sps - prop_zgnd) / (sps + prop_zgnd);
		q = abq_alos(r);
		q = f64::min(q, 1.0);

		if q < 0.25 || q < sps {
			r = r * (sps / q).sqrt();
		}
		q = prop.wn * prop.he[0] * prop.he[1] / (pd * 3.1415926535897);

		if prop.mdp < 0 {
			q = prop.wn * ((ht - hrp) * (hr - hrp)) / (pd *
								   3.1415926535897);
		}
		q -= q.floor();

		if q < 0.5 {
			q *= 3.1415926535897;
		}

		else {
			q = (1.0 - q) * 3.1415926535897;
		}
		/* no longer valid complex conjugate removed 
		   by removing minus sign from in front of sin function */
        re = abq_alos(Complex::new(q.cos(), q.sin()) + r);
		alosv = -10.0 * re.log10();
		prop.tgh = prop.hg[0];	/*tx above gnd hgt set to antenna height AGL */
		prop.tsgh = prop.rch[0] - prop.hg[0];	/* tsgh set to tx site gl AMSL */

		if (prop.hg[1] < prop.cch) && (prop.thera < 0.785)
		    && (prop.thenr < 0.785) {
			if sps < 0.05 {
				alosv = alosv + saalos(pd, prop, propa);
			} else {
				alosv = saalos(pd, prop, propa);
			}
		}
	}
	alosv = f64::min(22.0, alosv);
	return alosv;
}

pub(crate) fn abq_alos(r : Complex::<f64>) -> f64
{
	return r.re * r.re + r.im * r.im;
}

pub(crate) fn aknfe(v2: f64) -> f64
{
	let mut a;

	if v2 < 5.76 {
		a = 6.02 + 9.11 * sqrt(v2) - 1.27 * v2;
    }
	else {
		a = 12.953 + 10.0 * log10(v2);
    }
	return a;
}