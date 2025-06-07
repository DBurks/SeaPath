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