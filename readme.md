# Road Intersection Traffic Simulation
## Overview

This project simulates traffic control at a road intersection in a city. The primary objective is to manage the flow of vehicles through the intersection using traffic lights and ensure that traffic congestion is minimized while avoiding collisions.

## Objectives
   * Create a traffic control strategy.
   * Visualize the strategy with a simulation.
   * Use the SDL2 library for visualization.

## Environment and Rules
   ### Roads

   * Two roads crossing each other, forming an intersection.
   * Each road has one lane in each direction.
   * Vehicles can turn left, turn right, or go straight.

   ### Traffic Lights

   * Positioned at the points where each lane enters the intersection.
   * Have only two states: red and green.
   * Control algorithm to minimize congestion (not more than 8 vehicles) and avoid collisions.

   ### Vehicles

   * Painted in colors indicating their route.
   * Fixed velocity.
   * Maintain a safe distance from other vehicles.
   * Stop at red lights and proceed on green.

   ### Controls

   * ↑ Up: Spawn a vehicle moving from the south.
   * ↓ Down: Spawn a vehicle moving from the north.
   * → Right: Spawn a vehicle moving from the west.
   * ← Left: Spawn a vehicle moving from the east.
   * r: Spawn a vehicle from a random direction.
   * Esc: End the simulation.

## Notions

   - Documentation for SDL2.

## Getting Started
### Prerequisites

   - Rust programming language installed
   - SDL2 library

### Installation

    1. Clone the repository:
   ```sh
   git clone https://github.com/moussadiengsala/road_intersection.git
   cd road_intersection
   ```

    2. Install dependencies:

   ```sh
    cargo build
   ```

    3. Usage
   ```sh
    cargo run
   ```

   Use the keyboard controls to spawn vehicles and observe the traffic flow.


## Demo

[video demo](demo/demo.webm)

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## License
This project is licensed under the MIT License.

## Authors

- [@Moussa Dieng](https://www.moussa-dieng.dev)