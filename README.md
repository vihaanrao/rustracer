# Rustracer

## Overview

`Rustracer` is a powerful and performant ray tracer developed using the Rust programming language. This project is intended to illustrate the functionality of ray tracing algorithms while showcasing the strengths of Rust in creating complex, high-performance software.

## What is Ray Tracing?

Ray tracing is a rendering technique used in computer graphics to generate an image by tracing the path of light as pixels in an image plane and simulating the effects of its encounters with virtual objects. The technique is capable of producing a very high degree of visual realism, usually higher than that of typical scanline rendering methods.

## Why Rust?

We chose Rust for this project due to its speed, reliability, and rich type system. Rust's performance makes it an ideal choice for a computational task such as ray tracing. Furthermore, Rust's emphasis on safety (especially memory safety without a garbage collector) helps create robust software.

## Features

The `Rustracer` includes the following features:

- **Rendering:** The ray tracer is capable of rendering 3D scenes composed of spheres with realistic lighting and reflections.
- **Visual Effects:** Additional features include shadows, reflections, and refraction for more realistic visuals.
- **Performance:** The ray tracer has been optimized for performance and can handle complex scenes efficiently.
- **Output:** The rendered images can be displayed in a window or saved to a file for later viewing.

## Project Structure

The `Rustracer` is organized into several key modules:

- **Scene Module:** Defines the 3D scene to be rendered, including the camera, objects, and lighting.
- **Ray Module:** Handles the ray tracing algorithm and manages intersections between rays and objects in the scene.
- **Output Module:** Manages the output of the rendered image, supporting both display in a window or saving it to a file.
- **Optimization Module:** Optimizes the ray tracer for performance, using techniques such as multithreading and spatial data structures.

## Getting Started

To get started with the `Rustracer`, please refer to the `RUN.md` file in this repository. This includes instructions for setting up your environment, building the project, and running the ray tracer.

## License

The `Rustracer` is open-source software licensed under the MIT license. For more information, please see the `LICENSE.md` file.

We hope you enjoy exploring and using the `Rustracer`!
