#include "seapath/calculations.h"
#include <cmath>

namespace seapath
{
    namespace calculations
    {
        seapath::units::Distance great_circle_distance(const geodesy::LatLon &p1, const geodesy::LatLon &p2)
        {
            // Convert latitudes and longitudes to radians for calculations
            double lat1_rad = p1.latitude.radians;
            double lon1_rad = p1.longitude.radians;
            double lat2_rad = p2.latitude.radians;
            double lon2_rad = p2.longitude.radians;

            // Differences in latitude and longitude
            double delta_lat_rad = lat2_rad - lat1_rad;
            double delta_lon_rad = lon2_rad - lon1_rad;

            // Haversine formula
            double a = std::sin(delta_lat_rad / 2.0) * std::sin(delta_lat_rad / 2.0) +
                       std::cos(lat1_rad) * std::cos(lat2_rad) *
                           std::sin(delta_lon_rad / 2.0) * std::sin(delta_lon_rad / 2.0);

            double c = 2.0 * std::atan2(std::sqrt(a), std::sqrt(1.0 - a));

            // Earth's mean radius (using WGS84 semi-major axis as approximation for spherical model)
            // We convert meters to nautical miles within the Distance constructor.
            double earth_radius_meters = geodesy::Ellipsoid::WGS84.getMeanRadius(); // In meters

            double distance_meters = earth_radius_meters * c;

            // Construct a Distance object, which handles the conversion to nautical miles
            return seapath::units::Distance(distance_meters / 1852.0); // 1 nautical mile = 1852 meters
        }

        // You can implement great_circle_initial_bearing here later in a separate story.
        seapath::units::Angle great_circle_initial_bearing(const geodesy::LatLon &p1, const geodesy::LatLon &p2)
        {
            // Placeholder implementation for now.
            // This calculation is more complex than distance and can be done in a follow-up story.
            return seapath::units::Angle(0.0); // Return 0 degrees for now
        }

    } // namespace calculations

} // namespace seapath
