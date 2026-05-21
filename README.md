# SeaPath

SeaPath is a high-performance, real-time Guidance, Navigation, and Control (GNC) executive engine built for marine engineering applications. Designed with a strict separation of concerns, it decouples core geodetic and hydrodynamic physics tracking from low-level hardware abstraction layers, enabling seamless execution across host emulators (WSL/Linux) and embedded targets (BeagleBone Black).

## Architecture Overview

The system utilizes a modular, profile-driven real-time scheduler designed to execute deterministic tasks at precise update rates (e.g., 50 Hz frame budgets).

* Executive Scheduler: Drives task profiles deterministically using high-resolution platform timers.
* Geodetic Engine (libseapath_lib.so): Handles WGS-84 ellipsoid modeling, coordinate translations, and Great Circle navigation calculations.
* Network I/O Layer: Implements zero-overhead, non-blocking POSIX UDP socket frames matching precise byte alignment boundaries for external state synchronization.

---

## Technical Specifications & Wire Layout

Telemetry and control demands are exchanged over the network stack via a tightly packed, 56-byte binary payload structure to guarantee cross-language uniformity (C++/Rust) and eliminate serialization latency.

| Offset (Bytes) | Field Name | Type | Unit / Description |
| :--- | :--- | :--- | :--- |
| 0 .. 7 | timestamp_ns | uint64_t | Monotonic system clock time-of-validity |
| 8 .. 11 | sequence_number | uint32_t | Frame index tracking for drop/stale checks |
| 12 .. 19 | latitude_deg | double | Geodetic Latitude |
| 20 .. 27 | longitude_deg | double | Geodetic Longitude |
| 28 .. 31 | current_depth_m | float | Sensor depth feedback |
| 32 .. 35 | heading_rad | float | Current Yaw angle |
| 36 .. 39 | speed_knots | float | Forward speed |
| 40 .. 43 | target_depth_m | float | Commanded depth target |
| 44 .. 47 | target_heading_rad | float | Commanded tracking heading |
| 48 .. 51 | stern_plane_deg | float | Actuator position command |
| 52 .. 55 | rudder_deg | float | Actuator position command |

---

## Getting Started

### Prerequisites

* CMake (3.15 or higher)
* C++20 compliant compiler (g++-11 or higher)
* GoogleTest framework (automatically fetched via FetchContent during configuration)

### Compilation

Build the core geodetic library, main runtime node, and test suites directly from the build directory:

  cd ~/SeaPath
  mkdir -p build && cd build
  cmake ..
  make -j$(nproc)

### Running the Test Matrix

SeaPath enforces rigid mathematical and structural correctness via an automated unit test suite covering angle conversions, geodetic parameters, and Great Circle routing symmetric verification.

Execute the test suite from the build root:

  ./tests/seapath_tests

---

## Hardware-in-the-Loop (HIL) Simulation Setup

The execution binary supports direct real-time interaction with external software-in-the-loop (SIL) simulators (such as the companion Shipyard_GNC Rust engine).

1. Ensure the simulator is running and broadcasting on port 8081.
2. Fire up the SeaPath real-time node from the build root:

  ./src/seapath_rt_node

The executive will open a non-blocking POSIX socket on port 8080, ingest physical kinematics feedback at 50 Hz, step internal models, and instantly stream actuator demands back down the wire.