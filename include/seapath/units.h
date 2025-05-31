#pragma once
#include <cmath>

namespace seapath {
namespace units {

struct Angle
{
    double degrees;
    double radians;

    explicit Angle(double deg)
        : degrees(deg),
          radians(deg * M_PI/180.0) {}
};

struct Distance
{
    double nautical_miles;
    double meters;

    explicit Distance(double nm)
        : nautical_miles(nm),
          meters(nm*1852.0) {}
};

struct Speed
{
  double knots;
  double meters_per_second;

  explicit Speed(double kts)
      : knots(kts), 
        meters_per_second(kts * 0.5144444444) {}
};

} // namespace units
} //namespace seapath