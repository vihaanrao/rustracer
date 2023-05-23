# Rustracer

## Overview

`Rustracer` is a powerful, modular, and performant ray tracer developed using the Rust programming language. This project is intended to illustrate the functionality of ray tracing algorithms while showcasing the strengths of Rust in creating complex, high-performance software. Designed with modularity as a core principle, each component of the ray tracer is encapsulated within its own module, allowing for easy updates and extensions.

## What is Ray Tracing?

Ray tracing is a rendering technique used in computer graphics to generate an image by tracing the path of light as pixels in an image plane and simulating the effects of its encounters with virtual objects. The technique is capable of producing a very high degree of visual realism, usually higher than that of typical scanline rendering methods.

## Why Rust?

Rust was chosen for this project due to its speed, reliability, and rich type system. Rust's performance makes it an ideal choice for a computational task such as ray tracing. Furthermore, Rust's emphasis on safety (especially memory safety without a garbage collector) contributes to the robustness of the software.

## Features

The `Rustracer` includes the following features:

- **Rendering:** The ray tracer is capable of rendering 3D scenes composed of spheres with realistic lighting and reflections.
- **Visual Effects:** Additional features include shadows, reflections, and refraction for more realistic visuals.
- **Performance:** The ray tracer has been optimized for performance and can handle complex scenes efficiently.
- **Output:** The rendered images can be displayed in a window or saved to a file for later viewing.

## Modularity

The project has been designed with modularity in mind. Each component is encapsulated in its own module, facilitating easy updates and enhancements to individual components without impacting the overall functionality. This design promotes code reuse and better organization.

## Project Structure

The `Rustracer` is organized into several key files:

- **hit.rs:** Handles the calculations related to intersections of rays with objects in the scene.
- **lib.rs:** The library file for the project, coordinating the various modules and their interactions.
- **main.rs:** The entry point of the program. It sets up the scene and initiates the rendering process.
- **material.rs:** Defines the materials that can be applied to objects in the scene, affecting how they interact with light.
- **output.ppm:** The output file where the rendered image is stored.
- **ppm.rs:** Handles the functionality related to outputting the rendered image in the PPM format.
- **ray.rs:** Defines the ray tracing algorithm and manages intersections between rays and objects in the scene.
- **sphere.rs:** Defines the sphere object and how it interacts with rays.
- **vec3.rs:** Provides a 3D vector class used throughout the project for various calculations.

## Getting Started

To get started with the `Rustracer`, please refer to the `RUN.md` file in this repository. This includes instructions for setting up your environment, building the project, and running the ray tracer.

## License

The `Rustracer` is open-source software licensed under the MIT license. For more information, please see the `LICENSE.md` file.

We hope you enjoy exploring and using the `Rustracer`!
