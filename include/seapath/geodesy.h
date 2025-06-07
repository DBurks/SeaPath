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
                : latitude(lat_deg), longitude(lon_deg)
            {
                // TODO: add normalization to [-90,90], [-180,180] if desired
            }

            double getLatitudeDegrees() const { return latitude.degrees; }
            double getLongitudeDegrees() const { return longitude.degrees; }

            // TODO: add in isValid to make sure it's in normal rage, if desired
        };

        struct Ellipsoid
        {
            // Semi-major axis (equatorial radius) in meters
            double semi_major_axis;

            // inverse flattening (1/f)
            // flattening (f) where f = (a - b) / a; a is semi-major, b is semi-minor
            // inverse flattening is a / (a - b)
            double inverse_flattening;

            // semi-minor axis (polar radius) in meters
            double semi_minor_axis;

            // flattening f; derived
            double flattening;

            // eccentricity squared e^2 = 2f - f^2
            double eccentricity_squared;

            Ellipsoid(double a, double inv_f)
                : semi_major_axis(a), inverse_flattening(inv_f),
                  flattening(1/inv_f), semi_minor_axis(semi_major_axis * (1.0 - 1/inv_f)),
                  eccentricity_squared((2.0*1/inv_f) - (1/inv_f* 1/inv_f))
            {
            }

            double getEquatorialRadius() const { return semi_major_axis; }
            double getPolarRadius() const { return semi_minor_axis; }
            double getMeanRadius() const { return (2*semi_major_axis + semi_minor_axis)/3; }
            double getFlattening() const { return flattening; }
            double getEccentricity() const { return sqrt(eccentricity_squared); }
            double getEccentricitySquared() const { return eccentricity_squared; }

            // Static constant for the WGS84 ellipsoid (most common for GPS)
            static const Ellipsoid WGS84;

            // TODO: Add other models for ellipsoid
        };

    } // namespace geodesy
} // namespace seapath
