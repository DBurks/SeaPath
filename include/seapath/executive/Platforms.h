#pragma once
#include <chrono>
#include <thread>
#include <time.h>
#include <iostream>

namespace seapath::executive {

// Target Backend 1: Physical BeagleBone Black (Requires PREEMPT_RT or standard POSIX priorities)
class PhysicalBBBPlatform {
public:
    bool initialize_rt_scheduler() {
        std::cout << "[TARGET] Initializing Real-Time POSIX environment...\n";
        // NOTE: If using PREEMPT_RT, this is where we would invoke
        // sched_setscheduler() to set SCHED_FIFO thread constraints.
        return true;
    }

    void sleep_until_next_frame(std::chrono::nanoseconds target_time_since_epoch) {
        struct timespec ts;
        ts.tv_sec = std::chrono::duration_cast<std::chrono::seconds>(target_time_since_epoch).count();
        ts.tv_nsec = (target_time_since_epoch % std::chrono::seconds(1)).count();
        
        // Accurate clock sleep that prevents thread timing drift
        clock_nanosleep(CLOCK_MONOTONIC, TIMER_ABSTIME, &ts, NULL);
    }
};

// Target Backend 2: Host Emulator Platform (WSL Sandbox)
class HostEmulatorPlatform {
public:
    bool initialize_rt_scheduler() {
        std::cout << "[EMULATOR] Running in Software-in-the-Loop Mock Environment\n";
        return true;
    }

    void sleep_until_next_frame(std::chrono::nanoseconds target_time_since_epoch) {
        // Standard fall-through thread sleep for host testing without real-time kernel modules
        std::this_thread::sleep_until(
            std::chrono::time_point<std::chrono::steady_clock>(target_time_since_epoch)
        );
    }
};

} // namespace seapath::executive