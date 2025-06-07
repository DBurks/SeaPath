#include <gtest/gtest.h>
#include "seapath/geodesy.h"
#include"seapath/units.h"

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

class LatLonTest : public ::testing::Test {
protected:
    void SetUp() override {
        // basic setup code
    }
    void TearDown() override {
        // basic Tear down code
    }
};

TEST_F(LatLonTest, DefaultConstructorInitializesToZero) {
    seapath::geodesy::LatLon point;
    ASSERT_DOUBLE_EQ(point.getLatitudeDegrees(), 0.0);
    ASSERT_DOUBLE_EQ(point.getLongitudeDegrees(), 0.0);
}

TEST_F(LatLonTest, ConstructorWithDegreesSetsCorrectValues) {
    const double deg_lat = 45.0;
    const double deg_long = -75.0;

    seapath::geodesy::LatLon point(deg_lat, deg_long);
    ASSERT_DOUBLE_EQ(point.getLatitudeDegrees(), deg_lat);
    ASSERT_DOUBLE_EQ(point.getLongitudeDegrees(), deg_long);
}

// Test case for specific geographic points
TEST_F(LatLonTest, SpecificGeographicPoints) {
    // North Pole
    seapath::geodesy::LatLon north_pole(90.0, 0.0);
    ASSERT_DOUBLE_EQ(north_pole.getLatitudeDegrees(), 90.0);
    ASSERT_DOUBLE_EQ(north_pole.getLongitudeDegrees(), 0.0);

    // South Pole
    seapath::geodesy::LatLon south_pole(-90.0, 100.0); // Longitude doesn't matter at poles
    ASSERT_DOUBLE_EQ(south_pole.getLatitudeDegrees(), -90.0);
    ASSERT_DOUBLE_EQ(south_pole.getLongitudeDegrees(), 100.0); // Check that longitude is retained

    // Equator, Prime Meridian
    seapath::geodesy::LatLon origin(0.0, 0.0);
    ASSERT_DOUBLE_EQ(origin.getLatitudeDegrees(), 0.0);
    ASSERT_DOUBLE_EQ(origin.getLongitudeDegrees(), 0.0);

    // Equator, Date Line
    seapath::geodesy::LatLon international_dateline(0.0, 180.0);
    ASSERT_DOUBLE_EQ(international_dateline.getLatitudeDegrees(), 0.0);
    ASSERT_DOUBLE_EQ(international_dateline.getLongitudeDegrees(), 180.0);
}

// Test internal Angle objects (checking radians as well)
TEST_F(LatLonTest, InternalAngleObjectsAreCorrect) {
    seapath::geodesy::LatLon point(90.0, 0.0); // North Pole
    ASSERT_NEAR(point.latitude.radians, M_PI / 2.0, 1e-9); // 90 deg = PI/2 rad
    ASSERT_NEAR(point.longitude.radians, 0.0, 1e-9);

    seapath::geodesy::LatLon point_neg_180(0.0, -180.0);
    ASSERT_NEAR(point_neg_180.latitude.radians, 0.0, 1e-9);
    ASSERT_NEAR(point_neg_180.longitude.radians, -M_PI, 1e-9); // -180 deg = -PI rad
}

class EllipsoidTest : public ::testing::Test
{
protected:
    void SetUp() {

    }
    void TearDown() {

    }
    const seapath::geodesy::Ellipsoid& wgs84 = 
        seapath::geodesy::Ellipsoid::WGS84;
};

TEST_F(EllipsoidTest, WGSParametersAreCorrect) {
    ASSERT_DOUBLE_EQ(wgs84.getEquatorialRadius(),  6378137.0);
    double expected_semi_minor_axis = 6378137.0 * (1.0 - (1.0 / 298.257223563));
    ASSERT_DOUBLE_EQ(wgs84.getPolarRadius(),  expected_semi_minor_axis);
    double expected_mean_radius = (2 * 6378137.0 + expected_semi_minor_axis) / 3;
    ASSERT_DOUBLE_EQ(wgs84.getMeanRadius(),  expected_mean_radius);
    ASSERT_NEAR(wgs84.getFlattening(),  0.003352811, 1e-9);
    ASSERT_DOUBLE_EQ(wgs84.inverse_flattening, 298.257223563);
    ASSERT_NEAR(wgs84.getEccentricity(),  0.081819191, 1e-9);
    ASSERT_NEAR(wgs84.getEccentricitySquared(),  0.006694380, 1e-9);
}

TEST_F(EllipsoidTest, CustomEllipsoidCalculations) {
    // Example: A hypothetical perfect sphere (flattening = 0, inv_f = infinity)
    // In practice, define a very large inverse_flattening for near-sphere.
    // For a perfect sphere, semi_major_axis == semi_minor_axis.
    seapath::geodesy::Ellipsoid sphere(6371000.0, std::numeric_limits<double>::infinity());
    ASSERT_DOUBLE_EQ(sphere.semi_major_axis, 6371000.0);
    ASSERT_DOUBLE_EQ(sphere.semi_minor_axis, 6371000.0);
    ASSERT_DOUBLE_EQ(sphere.flattening, 0.0);
    ASSERT_DOUBLE_EQ(sphere.eccentricity_squared, 0.0);

    // Test a more exaggerated ellipsoid if needed
    seapath::geodesy::Ellipsoid exaggerated(10000.0, 10.0); // Very flat
    ASSERT_DOUBLE_EQ(exaggerated.semi_major_axis, 10000.0);
    ASSERT_DOUBLE_EQ(exaggerated.inverse_flattening, 10.0);
    ASSERT_NEAR(exaggerated.flattening, 0.1, 1e-12);
    ASSERT_NEAR(exaggerated.semi_minor_axis, 9000.0, 1e-9);
    ASSERT_NEAR(exaggerated.eccentricity_squared, 0.19, 1e-12); // 2*0.1 - 0.1*0.1 = 0.2 - 0.01 = 0.19
}
