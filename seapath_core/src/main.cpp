#include "seapath/executive/Scheduler.h"
#include "seapath/executive/Platforms.h"
#include "seapath/executive/NetworkIOTask.h"
#include <iostream>
#include <vector>
#include <memory>

using namespace seapath::executive;

int main() {
    std::cout << "=== SeaPath Navigation Node Initializing ===\n";

    HostEmulatorPlatform current_platform;
    Scheduler<HostEmulatorPlatform> scheduler(current_platform);

    // Build the execution profile containing only our live I/O driver task
    auto* live_profile = new std::vector<std::shared_ptr<ITask>>();
    live_profile->push_back(std::make_shared<NetworkIOTask>("UDP_Network_IO"));

    scheduler.submit_new_schedule(live_profile);
    
    std::cout << "[SYSTEM] Starting scheduler loop. Press Ctrl+C to terminate.\n";
    scheduler.start();

    return 0;
}