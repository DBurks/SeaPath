#include <iostream>
#include "seapath/units.h"

int main() {
    seapath::units::Angle my_angle(45.0);
    std::cout << "Angle: " << my_angle.degrees 
              << " degrees(" << my_angle.radians
              << " radians)" << std::endl;

    seapath::units::Distance my_distance(10.0);
    std::cout << "Distance: " 
              <<  my_distance.nautical_miles 
              << " nautical miles (" << my_distance.meters 
              << " meters)" << std::endl;

    std::cout << "SeaPath library basic usage example" << std::endl;

    return 0;
}