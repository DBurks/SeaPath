#pragma once

#include "seapath/units.h"
#include <string>
#include <cmath>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif


namespace seapath
{
namespace geodesy
{
        
struct LatLon
{
    seapath::units::Angle latitude;
    seapath::units::Angle longitude;

    // construct and object at 0 lat. 0 long
    LatLon() : latitude(0.0), longitude(0.0) {}

    // construct and object with the given lat long
    LatLon(double lat_deg, double lon_deg)
        : latitude(lat_deg), longitude(lon_deg) {
            // TODO: add normalization to [-90,90], [-180,180] if desired
    }

    double getLatitudeDegrees() const { return latitude.degrees; }
    double getLongitudeDegrees() const { return longitude.degrees; }

    // TODO: add in isValid to make sure it's in normal rage, if desired
};

} // namespace geodesy    
} // namespace seapath

