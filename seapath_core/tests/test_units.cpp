#include <gtest/gtest.h>
#include "seapath/units.h"

TEST(UnitsTest, AngleConversions) {
    seapath::units::Angle angle_90(90.0);
    ASSERT_NEAR(angle_90.degrees, 90.0, 1e-9);
    ASSERT_NEAR(angle_90.radians, M_PI / 2.0, 1e-9);

    seapath::units::Angle angle_180(180.0);
    ASSERT_NEAR(angle_180.degrees, 180.0, 1e-9);
    ASSERT_NEAR(angle_180.radians, M_PI, 1e-9);
}

TEST(UnitsTest, DistanceConversions) {
    seapath::units::Distance dist_1_nm(1.0);
    ASSERT_NEAR(dist_1_nm.nautical_miles, 1.0, 1e-9);
    ASSERT_NEAR(dist_1_nm.meters, 1852.0, 1e-9);

    seapath::units::Distance dist_half_nm(0.5);
    ASSERT_NEAR(dist_half_nm.nautical_miles, 0.5, 1e-9);
    ASSERT_NEAR(dist_half_nm.meters, 926.0, 1e-9);
}

TEST(UnitsTest, SpeedConversions) {
    seapath::units::Speed speed_1_kt(1.0);
    ASSERT_NEAR(speed_1_kt.knots, 1.0, 1e-9);
    ASSERT_NEAR(speed_1_kt.meters_per_second, 0.5144444444, 1e-9);

    seapath::units::Speed speed_50_kt(50.0);
    ASSERT_NEAR(speed_50_kt.knots, 50.0, 1e-9);
    ASSERT_NEAR(speed_50_kt.meters_per_second, 25.7222222222, 1e-8);
}