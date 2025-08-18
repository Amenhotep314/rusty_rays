# Rusty Rays
This is a port of my [raytracing project](https://github.com/Amenhotep314/ray_tracer), originally prototyped in Python and ported to Java. I decided to use this project as a chance to learn a slightly lower-level programming language, and picked Rust over C++.

## Current Features
When fully ported from its Java version, this raytracer will have these features:
 - Rendering of still images with spherical lightsources and background light.
 - Spherical bodies. Spheres can be controlled for reflectivity, refractivity, and specularity.
 - Planes. Planes can be solid or checkered, in any color. Their orientation is a work in progress.
 - Camera with customizable position, resolution, and defocus blur/depth of field.
 - Rudimentary animation capabilities.

## Goal Features (Maybe)
I hope this new implementation will open the door for more features, including
 - Better CPU performance through good code and parallelization
 - GPU acceleration with compute shaders
 - Fixed refraction logic, better support for materials
 - More robust geometric primitives, like cubes
 - Triangle-ray intersections and STL reading
 - BVH?
 - Volumes like smoke and fog
 - More robust animation support and features like motion blur
 - Spectral raycasting
 - Integration with physics engine

## Module Overview
- [main.rs](./main.rs) - Contains the primary functionality and algorithm for CPU-based raytracing
- [vector3.rs](./vector3.rs) - Implements useful structs and functions for dealing with 3D vector calculus, including Vector3, Ray, and RayHit.