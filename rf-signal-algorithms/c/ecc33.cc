#include <stdio.h>
#include <stdlib.h>
#include <math.h>

double ECC33pathLoss(float f, float TxH, float RxH, float d, int mode)
{

	// Sanity check as this model operates within limited Txh/Rxh bounds
	if(TxH-RxH<0){
		RxH=RxH/(d*2);
	}

/*	if (f < 700 || f > 3500) {
		fprintf(stderr,"Error: ECC33 model frequency range 700-3500MHz\n");
		exit(EXIT_FAILURE);
	}
*/
	// MHz to GHz
	f = f / 1000;

	double Gr = 0.759 * RxH - 1.862;	// Big city with tall buildings (1)
	// PL = Afs + Abm - Gb - Gr
	double Afs = 92.4 + 20 * log10(d) + 20 * log10(f);
	double Abm =
	    20.41 + 9.83 * log10(d) + 7.894 * log10(f) +
	    9.56 * (log10(f) * log10(f));
	double Gb = log10(TxH / 200) * (13.958 + 5.8 * (log10(d) * log10(d)));
	if (mode > 1) {		// Medium city (Europe)
		Gr = (42.57 + 13.7 * log10(f)) * (log10(RxH) - 0.585);
	}

	return Afs + Abm - Gb - Gr;
}