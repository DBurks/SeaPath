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