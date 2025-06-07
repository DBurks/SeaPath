#pragma once

#include "seapath/units.h"
#include "seapath/geodesy.h"

namespace seapath
{
    namespace calculations
    {
        /**
         * @brief Calculates the great circle distance between two geographic points
         * using the Haversine formula on a spherical Earth model.
         * @param p1 The first geographic point (LatLon).
         * @param p2 The second geographic point (LatLon).
         * @return The distance in nautical miles (as seapath::units::Distance).
         */
        seapath::units::Distance great_circle_distance(const geodesy::LatLon &p1, const geodesy::LatLon &p2);

        /**
         * @brief Calculates the initial great circle bearing from point p1 to p2.
         * @param p1 The first geographic point (LatLon).
         * @param p2 The second geographic point (LatLon).
         * @return The initial bearing in degrees (as seapath::units::Angle).
         */
        seapath::units::Angle great_circle_initial_bearing(const geodesy::LatLon &p1, const geodesy::LatLon &p2);

    } // namespace calculations

} // namespace seapath
