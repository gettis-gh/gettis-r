# gettis-r
Getti's Renderer

## Implementations

**Rendering Techniques:**
- Rasterization
- Raytracing
- Raymarching (Sphere Tracing)

**Drawing Techniques:**
- Bitmap writting
- Ascii writting
- Desktop Window Drawing

**Object types:**
- **SDF (Signed Distance Function)**: Used by the Raymarcher, which get the color of the closest SDF Object after the colission.
- **3d Meshes**: An efficient way to store the triangles which just stores vertices and triangle's indices instead of independent triangles to avoid vertices redundancy.