#pragma once
#include "Task.h"
#include <vector>
#include <memory>
#include <atomic>
#include <chrono>
#include <thread>
#include <iostream>

namespace seapath::executive {

// C++20 Concept to enforce compliance for our target platform backends
template<typename T>
concept RealTimePlatform = requires(T platform, std::chrono::nanoseconds ns) {
    { platform.initialize_rt_scheduler() } -> std::same_as<bool>;
    { platform.sleep_until_next_frame(ns) } -> std::same_as<void>;
};

template<RealTimePlatform PlatformImpl>
class Scheduler {
private:
    std::vector<std::shared_ptr<ITask>> active_schedule;
    std::atomic<std::vector<std::shared_ptr<ITask>>*> pending_schedule{nullptr};
    
    PlatformImpl platform;
    std::atomic<bool> running{false};
    const std::chrono::nanoseconds frame_duration{20000000}; // 20ms (50 Hz)

public:
    Scheduler(PlatformImpl p) : platform(std::move(p)) {}

    void submit_new_schedule(std::vector<std::shared_ptr<ITask>>* new_sched) {
        std::vector<std::shared_ptr<ITask>>* old_pending = pending_schedule.exchange(new_sched, std::memory_order_acq_rel);
        if (old_pending) {
            delete old_pending; 
        }
    }

    void start() {
        if (!platform.initialize_rt_scheduler()) {
            std::cerr << "[CRITICAL] Failed to initialize real-time environment.\n";
            return;
        }
        running = true;
        execution_loop();
    }

    void stop() {
        running = false;
    }

private:
    void execution_loop() {
        auto next_frame = std::chrono::steady_clock::now();

        while (running.load(std::memory_order_relaxed)) {
            next_frame += frame_duration;

            // 1. Single-Frame Boundary Swap
            if (pending_schedule.load(std::memory_order_acquire) != nullptr) {
                auto* incoming = pending_schedule.exchange(nullptr, std::memory_order_acq_rel);
                if (incoming) {
                    for (auto& task : active_schedule) task->on_stop();
                    active_schedule = std::move(*incoming);
                    delete incoming;
                    for (auto& task : active_schedule) task->on_start();
                }
            }

            // 2. Execute Orthogonal Tasks
            for (auto& task : active_schedule) {
                task->update();
            }

            // 3. Precision Sleep (delegated to the platform target)
            platform.sleep_until_next_frame(next_frame.time_since_epoch());
        }
    }
};

} // namespace seapath::executive