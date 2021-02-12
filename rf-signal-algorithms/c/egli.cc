/*****************************************************************************
*  Egli VHF/UHF model for Signal Server by G6DTX                             *
*  April 2017                                                                *
*  This program is free software; you can redistribute it and/or modify it   *
*  under the terms of the GNU General Public License as published by the     *
*  Free Software Foundation; either version 2 of the License or any later    *
*  version.                                                                  *
*                                                                            *
*  This program is distributed in the hope that it will useful, but WITHOUT  *
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or     *
*  FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License v3  *
*  for more details.                                                         *
******************************************************************************

Frequency 30 to 1000MHz
h1 = 1m and above
h2 = 1m and above
Distance 1 to 50km

http://people.seas.harvard.edu/~jones/es151/prop_models/propagation.html#pel
*/

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

//static float fcmin = 30.0;
//static float fcmax = 1000.0;
//static float dmin  = 1.0;
//static float dmax  = 50.0;
//static float h1min = 1.0;
//static float h2min = 1.0;

static __inline float _10log10f(float x)
{
  return(4.342944f*logf(x));
}

double EgliPathLoss(float f, float h1, float h2, float d)
{
  double Lp50 = NAN;
  float C1, C2;

/*  if ((f >= fcmin) && (f <= fcmax) &&
      (h1 >= h1min) && (h2 >= h2min))
  {*/
    if (h1 > 10.0 && h2 > 10.0)
    {
      Lp50 = 85.9;
      C1 = 2.0;
      C2 = 2.0;
    }
    else if (h1 > 10.0)
    {
      Lp50 = 76.3;
      C1 = 2.0;
      C2 = 1.0;
    }
    else if (h2 > 10.0)
    {
      Lp50 = 76.3;
      C1 = 1.0;
      C2 = 2.0;
    }
    else // both antenna heights below 10 metres
    {
      Lp50 = 66.7;
      C1 = 1.0;
      C2 = 1.0;
    } // end if

    Lp50 += 4.0f*_10log10f(d) + 2.0f*_10log10f(f) - C1*_10log10f(h1) - C2*_10log10f(h2);
  /*}
  else
  {
    fprintf(stderr,"Parameter error: Egli path loss model f=%6.2f h1=%6.2f h2=%6.2f d=%6.2f\n", f, h1, h2, d);
    exit(EXIT_FAILURE);
  }*/

  return(Lp50);

}
