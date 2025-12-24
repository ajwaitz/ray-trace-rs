# ray-trace-rs

A ray tracer implemented in Rust with support for both standalone binary and WebAssembly.

Following [this tutorial](https://raytracing.github.io/books/RayTracingInOneWeekend.html#overview).

## Project Structure

This is a Cargo workspace with three crates:

- **`ray-tracer-core`** - Core ray tracing library (threading-agnostic)
- **`ray-tracer-cli`** - Standalone binary with multi-threading support  
- **`ray-tracer-wasm`** - WebAssembly bindings for web deployment

## Usage

### 1. CLI Binary (Multi-threaded Ray Tracer)

The CLI version renders images using multiple threads for faster performance.

**Build:**
```bash
cargo build --release -p ray-tracer-cli
```

**Run:**
```bash
cargo run -p ray-tracer-cli
# or
./target/release/ray-tracer
```

**Output:** Creates `test.ppm` file in the project root (PPM format, 512x512 pixels)

**Features:**
- Multi-threaded rendering (16 threads by default)
- Supports spheres, triangles, and polygon meshes (.obj files)
- Materials: Lambertian (diffuse) and Metal (reflective)
- Anti-aliasing (10 samples per pixel)
- Recursive ray tracing (max depth 10)

### 2. WebAssembly (Browser-based Ray Tracer)

The WASM version runs in the browser and returns PNG images.

**Build:**
```bash
# Install wasm-pack if not already installed
cargo install wasm-pack

# Build WASM package (outputs to root pkg/ directory)
wasm-pack build ray-tracer-wasm --target web --out-dir ../pkg
```

**Run:**
```bash
# Start a local web server (required for WASM modules)
# Option 1: Python
python3 -m http.server 8000

# Option 2: Node.js
npx serve

# Option 3: Rust
cargo install basic-http-server
basic-http-server
```

Then open `http://localhost:8000/index.html` in your browser.

**JavaScript API:**
```javascript
import init, { render } from "./pkg/ray_tracer_wasm.js";

await init();
const imageBytes = render(1n); // 1n = samples per pixel (BigInt)
// imageBytes is a Uint8Array containing PNG data
```

**Alternative API (WasmRayTracer struct):**
```javascript
import init, { WasmRayTracer } from "./pkg/ray_tracer_wasm.js";

await init();
const tracer = new WasmRayTracer();
const imageBytes = tracer.render(); // Returns PNG bytes
const width = tracer.get_width();
const height = tracer.get_height();
```

### 3. Core Library (As a Dependency)

Use the core library in your own Rust projects.

**In your `Cargo.toml`:**
```toml
[dependencies]
ray-tracer-core = { path = "../ray-trace-rs/ray-tracer-core" }
```

**Example usage:**
```rust
use ray_tracer_core::*;
use std::sync::Arc;

fn main() {
    let camera = Camera::new();
    let mut world = HittableList::new();
    
    // Create materials
    let material: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    
    // Add objects
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, 0.0, -1.0),
        0.5,
        &material,
    )));
    
    // Render single-threaded
    let pixels = camera.render_single_threaded(&world);
    // pixels is Vec<Vec3> containing color data
}
```

**Core Library Features:**
- Threading-agnostic design
- `Camera::render_single_threaded()` - Returns `Vec<Vec3>` of pixel colors
- `Camera::render_pixel()` - Render individual pixels
- All geometry types: `Sphere`, `Triangle`, `Polygon`
- Material system: `Lambertian`, `Metal`

## Building

### CLI Binary
```bash
cargo build --release -p ray-tracer-cli
```

### WebAssembly
```bash
# Build from project root (outputs to root pkg/ directory)
wasm-pack build ray-tracer-wasm --target web --out-dir pkg
```

### Core Library Only
```bash
cargo build -p ray-tracer-core
```

### All Crates
```bash
cargo build --release
```

## Assets

- `shape.obj` - 3D model file for polygon rendering
- `test.ppm` - Generated output image (PPM format) from CLI
- `index.html` - Web interface for WASM version
- `pkg/` - WASM build output directory