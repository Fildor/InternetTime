#include <stdio.h>
#include <math.h>
#include <time.h>

#define BMT (+1) // Biel Mean Time w/o daylight savings

double get_beats( int utcSecs, int utcMins, int utcHrs )
{
	double value = 0.0;
	value += utcSecs;
	value += utcMins*60;
	value += ((utcHrs+BMT)%24)*3600;
	return value/86.4;
}

int main()
{
	time_t now;
	struct tm *utcNow;

	time(&now);
	utcNow = gmtime(&now);
	double beats = get_beats(utcNow->tm_sec, utcNow->tm_min, utcNow->tm_hour);
	printf("@%.1lf", beats);
        return 0;
}
