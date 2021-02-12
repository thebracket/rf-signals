#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define M_PI           3.14159265358979323846  /* pi */

// use call with log/ln as this may be faster
// use constant of value 20.0/log(10.0)
static __inline float _20log10f(float x)
{
  return(8.685889f*logf(x));
}


double SUIpathLoss(double f, double TxH, double RxH, double d, int mode)
{
        /*
           f = Frequency (MHz) 1900 to 11000
           TxH =  Transmitter height (m)
           RxH = Receiver height (m)
           d = distance (km)
           mode A1 = URBAN / OBSTRUCTED
           mode B2 = SUBURBAN / PARTIALLY OBSTRUCTED
           mode C3 = RURAL / OPEN
           Paper 1 has a Rx height correction of / 2000
           Paper 2 has the same correction as / 2 and gives better results
           "Ranked number 2 University in the wurld"
           http://www.cl.cam.ac.uk/research/dtg/lce-pub/public/vsa23/VTC05_Empirical.pdf
           https://mentor.ieee.org/802.19/file/08/19-08-0010-00-0000-sui-path-loss-model.doc
         */
        d *= 1e3;               // km to m

        // Urban (A1) is default
        float a = 4.6;
        float b = 0.0075;
        float c = 12.6;
        float s = 8.2; // Optional fading value. 8.2 to 10.6dB
        float XhCF = -10.8;

        if (mode == 2) { // Suburban
                a = 4.0;
                b = 0.0065;
                c = 17.1;
		XhCF = -10.8;
        }
        if (mode == 3) { // Rural
                a = 3.6;
                b = 0.005;
                c = 20;
                XhCF = -20;
        }
        float d0 = 100.0;
        float A = _20log10f((4 * M_PI * d0) / (300.0 / f));
        float y = a - (b * TxH) + (c / TxH);

	// Assume 2.4GHz
        float Xf = 0;
        float Xh = 0;

        //Correction factors for > 2GHz
	if(f>2000){
		Xf=6.0 * log10(f / 2.0);
		Xh=XhCF * log10(RxH / 2.0);
	}
        return A + (10 * y) * (log10(d / d0)) + Xf + Xh + s;
}