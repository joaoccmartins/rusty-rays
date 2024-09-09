# Rusty Rays [WIP]
This is a path tracer written in Rust for my personal amusement very much in early stages. Architecture looks to mix parts from the [Physically Based Rendering book](https://www.pbr-book.org/4ed/contents) and from [Ray Tracing in a weeken series](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

The project aims to be a library, but current implementation is a binary that uses the various library modules, and renders the default scene to a window using minifb. On exit it will save the current framebuffer of the renderer to a file called `new_image.png`

# Structure
- `renderer` module contains the core rendering functions, and two implementations of their usage, a `single_threaded` and simple `multi_threaded` versions.
- `scene_graph` module contains `material` and `primitive` definitions.
- `color` module contains some color utilities as well as a `Framebuffer` struct used to store pixel data, as well as export using the `image` crate.

