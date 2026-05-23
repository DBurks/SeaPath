# ARCH_SPEC.md

## Overview

This document outlines the technical specifications for the marine surface vessel Guidance, Navigation, and Control (GNC) system in Rust. The GNC system is responsible for guiding the vessel to its destination, navigating through its surroundings, and controlling its movements.

## Guidance

The guidance system uses the Vector Field Method (VFM) to follow a waypoint trajectory. The VFM is a control method that uses a vector field to guide the vessel to its destination.

### Vector Field Method (VFM)

The VFM is defined as:

$$\mathbf{u}(\mathbf{x}, t) = -\frac{\partial V}{\partial \mathbf{x}}(\mathbf{x}, t)$$

where $\mathbf{u}$ is the control input, $\mathbf{x}$ is the state of the vessel, $V$ is the potential function, and $t$ is time.

### Waypoint Following

The waypoint following algorithm is implemented as follows:

1. Define a set of waypoints that the vessel needs to follow.
2. At each time step, calculate the control input $\mathbf{u}$ using the VFM.
3. Update the state of the vessel using the control input.
4. Check if the vessel has reached the next waypoint. If so, move on to the next waypoint.

## Navigation

The navigation system uses a Kalman filter to fuse sensor data from various sources.

### Kalman Filter

The Kalman filter is implemented as follows:

1. Define the state transition matrix $\mathbf{F}$ and the measurement matrix $\mathbf{H}$.
2. Define the covariance matrix $\mathbf{P}$ and the gain matrix $\mathbf{K}$.
3. At each time step, update the state estimate $\hat{\mathbf{x}}$ using the Kalman filter equations:

$$\hat{\mathbf{x}}_{k+1} = \mathbf{F}\hat{\mathbf{x}}_k + \mathbf{B}\mathbf{u}_k$$

$$\mathbf{P}_{k+1} = \mathbf{F}\mathbf{P}_k\mathbf{F}^T + \mathbf{Q}$$

$$\mathbf{K}_k = \mathbf{P}_k\mathbf{H}^T(\mathbf{H}\mathbf{P}_k\mathbf{H}^T + \mathbf{R})^{-1}$$

$$\hat{\mathbf{x}}_{k+1} = \hat{\mathbf{x}}_k + \mathbf{K}_k(\mathbf{z}_k - \mathbf{H}\hat{\mathbf{x}}_k)$$

$$\mathbf{P}_{k+1} = (\mathbf{I} - \mathbf{K}_k\mathbf{H})\mathbf{P}_k$$

## Control

The control system uses a PID controller to control the movements of the vessel.

### PID Controller

The PID controller is implemented as follows:

1. Define the setpoint $r$ and the process variable $y$.
2. At each time step, calculate the control input $\mathbf{u}$ using the PID controller equations:

$$\mathbf{u} = K_p(e) + K_i\int e dt + K_d\frac{de}{dt}$$

where $e$ is the error between the setpoint and the process variable.

## Rust Implementation

The GNC system will be implemented in Rust as a set of structs and traits. The structs will represent the different components of the system, such as the guidance, navigation, and control systems. The traits will define the behavior of these components.

### Guidance Struct

```rust
pub struct Guidance {
    waypoints: Vec<[f64; 2]>,
    vfm: VecFieldMethod,
}

impl Guidance {
    pub fn new(waypoints: Vec<[f64; 2]>) -> Self {
        Guidance {
            waypoints,
            vfm: VecFieldMethod::new(),
        }
    }

    pub fn update(&mut self, state: &mut [f64]) {
        self.vfm.update(state);
    }
}
```

### Navigation Struct

```rust
pub struct Navigation {
    kalman_filter: KalmanFilter,
}

impl Navigation {
    pub fn new() -> Self {
        Navigation {
            kalman_filter: KalmanFilter::new(),
        }
    }

    pub fn update(&mut self, state: &mut [f64], measurement: &[f64]) {
        self.kalman_filter.update(state, measurement);
    }
}
```

### Control Struct

```rust
pub struct Control {
    pid_controller: PIDController,
}

impl Control {
    pub fn new() -> Self {
        Control {
            pid_controller: PIDController::new(),
        }
    }

    pub fn update(&mut self, state: &mut [f64], setpoint: &[f64]) {
        self.pid_controller.update(state, setpoint);
    }
}
```

### Traits

```rust
pub trait GuidanceTrait {
    fn update(&mut self, state: &mut [f64]);
}

pub trait NavigationTrait {
    fn update(&mut self, state: &mut [f64], measurement: &[f64]);
}

pub trait ControlTrait {
    fn update(&mut self, state: &mut [f64], setpoint: &[f64]);
}
```

Note that this is a simplified example and the actual implementation will depend on the specific requirements of the project.