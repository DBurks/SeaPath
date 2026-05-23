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

            LatLon() : latitude(0.0), longitude(0.0) {}

            LatLon(double lat_deg, double lon_deg)
                : latitude(lat_deg), longitude(lon_deg) {}

            double getLatitudeDegrees() const { return latitude.degrees; }
            double getLongitudeDegrees() const { return longitude.degrees; }
            double getLatitudeRadians() const { return latitude.degrees * (M_PI / 180.0); }
            double getLongitudeRadians() const { return longitude.degrees * (M_PI / 180.0); }

            bool isValid() const {
                return (latitude.degrees >= -90.0 && latitude.degrees <= 90.0) &&
                       (longitude.degrees >= -180.0 && longitude.degrees <= 180.0);
            }
        };

        struct Ellipsoid
        {
            double semi_major_axis;       // a (meters)
            double inverse_flattening;    // 1/f
            double semi_minor_axis;       // b (meters)
            double flattening;            // f
            double eccentricity_squared;  // e^2

            Ellipsoid(double a, double inv_f)
                : semi_major_axis(a),
                  inverse_flattening(inv_f),
                  flattening(1.0 / inv_f),
                  semi_minor_axis(a * (1.0 - (1.0 / inv_f))),
                  eccentricity_squared((2.0 * (1.0 / inv_f)) - ((1.0 / inv_f) * (1.0 / inv_f))) {}

            double getEquatorialRadius() const { return semi_major_axis; }
            double getPolarRadius() const { return semi_minor_axis; }
            double getMeanRadius() const { return (2.0 * semi_major_axis + semi_minor_axis) / 3.0; }
            double getFlattening() const { return flattening; }
            double getEccentricity() const { return std::sqrt(eccentricity_squared); }
            double getEccentricitySquared() const { return eccentricity_squared; }

            // Static declaration matching your original tracking model
            static const Ellipsoid WGS84;
        };

        // ====================================================================
        // Core Mathematical Functions (Declared AFTER Ellipsoid is complete)
        // ====================================================================

        /**
         * @brief Calculates the Meridional radius of curvature (North-South)
         */
        double calculate_meridional_radius(const LatLon& pos, const Ellipsoid& ell = Ellipsoid::WGS84);

        /**
         * @brief Calculates the Prime Vertical radius of curvature (East-West)
         */
        double calculate_prime_vertical_radius(const LatLon& pos, const Ellipsoid& ell = Ellipsoid::WGS84);

        /**
         * @brief Computes delta meters from a small delta coordinate change
         */
        void distance_deltas_from_ll(const LatLon& base, double delta_lat_deg, double delta_lon_deg, 
                                     double& out_dx_meters, double& out_dy_meters, 
                                     const Ellipsoid& ell = Ellipsoid::WGS84);

    } // namespace geodesy
} // namespace seapath