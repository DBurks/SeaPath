#include <gtest/gtest.h>
#include "seapath/calculations.h"
#include "seapath/units.h"
#include "seapath/geodesy.h"

class CalculationsTest : public ::testing::Test {
protected:
    // Define some common points for tests
    seapath::geodesy::LatLon greenwich;
    seapath::geodesy::LatLon new_york;
    seapath::geodesy::LatLon sydney;
    seapath::geodesy::LatLon north_pole;
    seapath::geodesy::LatLon south_pole;
    seapath::geodesy::LatLon equator_0_lon;
    seapath::geodesy::LatLon equator_90_lon;

    void SetUp() override {
        // Known geographic points
        greenwich = seapath::geodesy::LatLon(51.476852, -0.000500); // Approx. Greenwich Obs.
        new_york = seapath::geodesy::LatLon(40.730610, -73.935242); // Approx. New York City
        sydney = seapath::geodesy::LatLon(-33.8688, 151.2093); // Approx. Sydney, Australia
        north_pole = seapath::geodesy::LatLon(90.0, 0.0);
        south_pole = seapath::geodesy::LatLon(-90.0, 0.0);
        equator_0_lon = seapath::geodesy::LatLon(0.0, 0.0);
        equator_90_lon = seapath::geodesy::LatLon(0.0, 90.0);
    }
};

TEST_F(CalculationsTest, GreatCircleDistance_IdenticalPointsIsZero) {
    ASSERT_NEAR(seapath::calculations::great_circle_distance(greenwich, greenwich).nautical_miles, 0.0, 1e-9);
    ASSERT_NEAR(seapath::calculations::great_circle_distance(north_pole, north_pole).nautical_miles, 0.0, 1e-9);
}

TEST_F(CalculationsTest, GreatCircleDistance_Poles) {
    // Distance from North Pole to South Pole should be half Earth's circumference
    // Circumference = 2 * PI * R_mean
    // Half circumference = PI * R_mean
    // R_mean approx WGS84 semi_major_axis (6378137.0 meters)
    // Distance in NM = (PI * 6378137.0) / 1852.0
    double expected_distance_nm = (M_PI * seapath::geodesy::Ellipsoid::WGS84.getMeanRadius()) / 1852.0;
    ASSERT_NEAR(seapath::calculations::great_circle_distance(north_pole, south_pole).nautical_miles, expected_distance_nm, 0.1); // Allowing a bit more tolerance for approximation
}

// Test for distance along the equator (known distance)
TEST_F(CalculationsTest, GreatCircleDistance_AlongEquator) {
    // 90 degrees of longitude along equator should be 1/4 of equatorial circumference
    // Earth's circumference at equator ~ 40,075 km = 21639 NM
    // 1/4 of this is ~ 5409.75 NM
    // More precisely: (PI/2) * R_equator_meters / 1852
    double expected_distance_nm = (M_PI / 2.0 * seapath::geodesy::Ellipsoid::WGS84.getMeanRadius()) / 1852.0;
    ASSERT_NEAR(seapath::calculations::great_circle_distance(equator_0_lon, equator_90_lon).nautical_miles, expected_distance_nm, 0.1);
}

// Test for a known real-world route (e.g., London to New York)
// You'll need to look up these values or calculate them with an external tool.
TEST_F(CalculationsTest, GreatCircleDistance_LondonToNewYork) {
    // Approximate points: London (51.5, 0.0), New York (40.7, -74.0)
    // Online calculators give ~2996 NM for this route (spherical model).
    // Using more precise coordinates above for greenwich and new_york.
    // A quick online calc gives ~3007 for those points (WGS84 spherical)
    double expected_distance_nm = 3007.5657673251753; // Adjust based on your chosen reference
    ASSERT_NEAR(seapath::calculations::great_circle_distance(greenwich, new_york).nautical_miles, expected_distance_nm, 5.0); // Allow some tolerance
}

// Test for reverse direction (distance should be symmetric)
TEST_F(CalculationsTest, GreatCircleDistance_Symmetric) {
    double dist1 = seapath::calculations::great_circle_distance(greenwich, new_york).nautical_miles;
    double dist2 = seapath::calculations::great_circle_distance(new_york, greenwich).nautical_miles;
    ASSERT_DOUBLE_EQ(dist1, dist2); // Should be exactly equal
}

// Add more edge cases:
// - Points at same latitude, different longitude
// - Points at same longitude, different latitude
// - Points crossing the antimeridian (180th longitude) - Haversine handles this well.
// - Points very close to each other (check for numerical stability, Haversine is good here)