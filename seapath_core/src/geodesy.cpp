#include "seapath/geodesy.h"

namespace seapath
{
    namespace geodesy
    {
        // 1. Instantiates the static allocation so the linker can find it
        const Ellipsoid Ellipsoid::WGS84(6378137.0, 298.257223563);

        double calculate_meridional_radius(const LatLon& pos, const Ellipsoid& ell)
        {
            double lat_rad = pos.getLatitudeRadians();
            double sin_lat = std::sin(lat_rad);
            double denom = 1.0 - ell.eccentricity_squared * sin_lat * sin_lat;
            
            // M = a(1 - e^2) / (1 - e^2 sin^2(phi))^(3/2)
            return (ell.semi_major_axis * (1.0 - ell.eccentricity_squared)) / (denom * std::sqrt(denom));
        }

        double calculate_prime_vertical_radius(const LatLon& pos, const Ellipsoid& ell)
        {
            double lat_rad = pos.getLatitudeRadians();
            double sin_lat = std::sin(lat_rad);
            double denom = 1.0 - ell.eccentricity_squared * sin_lat * sin_lat;
            
            // N = a / sqrt(1 - e^2 sin^2(phi))
            return ell.semi_major_axis / std::sqrt(denom);
        }

        void distance_deltas_from_ll(const LatLon& base, double delta_lat_deg, double delta_lon_deg, 
                                     double& out_dx_meters, double& out_dy_meters, const Ellipsoid& ell)
        {
            double M = calculate_meridional_radius(base, ell);
            double N = calculate_prime_vertical_radius(base, ell);

            double dlat_rad = delta_lat_deg * (M_PI / 180.0);
            double dlon_rad = delta_lon_deg * (M_PI / 180.0);

            // dY = M * dLat (North-South distance)
            out_dy_meters = M * dlat_rad;
            
            // dX = N * cos(Lat) * dLon (East-West distance)
            out_dx_meters = N * std::cos(base.getLatitudeRadians()) * dlon_rad;
        }

    } // namespace geodesy
} // namespace seapath