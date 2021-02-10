#ifndef _COMMON_H_
#define _COMMON_H_

#define GAMMA 		2.5

#ifndef PI
  #define PI		3.141592653589793
#endif

#ifndef TWOPI
  #define TWOPI		6.283185307179586
#endif

#ifndef HALFPI
  #define HALFPI	1.570796326794896
#endif

#define DEG2RAD		1.74532925199e-02
#define	EARTHRADIUS	20902230.97
#define	METERS_PER_MILE 1609.344
#define	METERS_PER_FOOT 0.3048
#define	KM_PER_MILE	1.609344
#define	FEET_PER_MILE	5280.0
#define FOUR_THIRDS	1.3333333333333

#define MAX(x,y)((x)>(y)?(x):(y))

struct dem {
	float min_north;
	float max_north;
	float min_west;
	float max_west;
	int max_el;
	int min_el;
	short **data;
	unsigned char **mask;
	unsigned char **signal;
};

struct site {
	double lat;
	double lon;
	float alt;
	char name[50];
	char filename[255];
};

struct path {
	double *lat;
	double *lon;
	double *elevation;
	double *distance;
	int length;
};

struct LR {
	double eps_dielect;
	double sgm_conductivity;
	double eno_ns_surfref;
	double frq_mhz;
	double conf;
	double rel;
	double erp;
	int radio_climate;
	int pol;
	float antenna_pattern[361][1001];
};

struct region {
	unsigned char color[128][3];
	int level[128];
	int levels;
};

extern int MAXPAGES;
extern int ARRAYSIZE;
extern int IPPD;

extern double min_north;
extern double max_north;
extern double min_west;
extern double max_west;
extern int ippd;
extern int MAXRAD;
extern int mpi;
extern int max_elevation;
extern int min_elevation;
extern int contour_threshold;
extern int loops;
extern int jgets;
extern int width;
extern int height;

extern double earthradius;
extern double north;
extern double east;
extern double south;
extern double west;
extern double max_range;
extern double dpp;
extern double ppd;
extern double yppd;
extern double fzone_clearance;
extern double clutter;
extern double dBm;
extern double loss;
extern double field_strength;
//extern thread_local double *elev;
extern double westoffset;
extern double eastoffset;
extern double delta;
extern double cropLat;
extern double cropLon;

extern char string[];
extern char sdf_path[];
extern char gpsav;

extern unsigned char got_elevation_pattern;
extern unsigned char got_azimuth_pattern;
extern unsigned char metric;
extern unsigned char dbm;

extern struct dem *dem;
extern thread_local struct path path;
extern struct LR LR;
extern struct region region;

extern int debug;

#endif /* _COMMON_H_ */