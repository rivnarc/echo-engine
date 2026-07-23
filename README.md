<p align="center"><img src="src/assets/logo.png" alt="Echo Engine Logo" width="200" height=""200/></p>
<p align="center"><em>Where the ideas resonate.</em></p>
<h1 align="center">Echo Engine</h1>

> The project is in the early stages of development. Stay tuned for the official release.

Echo Engine — is completely new 3D and 2D engine, which is used to create games and some sort of simulations. 
## Technology Stack
* Language: [Rust](https://www.rust-lang.org/)
* Graphics API: [`wgpu`](https://wgpu.rs/) (Vulkan and DX12)
* Window Manager: [`winit`](https://crates.io/crates/winit)
## Roadmap
### Stage 1: Foundation and GPU
- [x] Repository and concept initialization
- [ ] Window and event handling (`winit`)
- [ ] `wgpu` context initialization
- [ ] Rendering the first triangle (WGSL shader)

### Stage 2: 2D Rendering
- [ ] Texture loading
- [ ] 2D Sprite Batcher
- [ ] Simple 2D camera setup

### Stage 3: Transition to 3D
- [ ] 3D math and perspective matrix
- [ ] 3D model loading
- [ ] Basic lighting
## Quick Start
Make sure you have installed [Rust](https://www.rust-lang.org/).

```bash
git clone [https://github.com/rivnarc/echo-engine.git](https://github.com/rivnarc/echo-engine.git)
```
```bash
cd echo-engine
```
```bash
cargo run
```
## License
Copyright © 2026 **rivnarc**.
All rights reserved.
Distributed under the license [Apache-2.0](LICENSE).