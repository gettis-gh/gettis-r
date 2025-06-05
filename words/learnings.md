# There I write the things I learned while working on this project

## Pipeline

This is one of the most important things I learned from this project. It started with some random Rendering Pipeline, because I didn't know a shi-. but now, at latest version, I implemented a more modular and clear one where steps like Triangulation, Rasterization and Drawing are separated, independent and replaceable. So now I dont have to change code from everywhere just to modify a step.

## Rasterization

I learned there's so many ways of "Pixelize" graphics. In my case, I've working on these two: 
- Pixel by pixel of the whole screen picking closest triangle color
- Pixel by pixel of every triangle storing triangle's color if theres not a closer one

## Triangulation

Not the process behind it, just the concept. I learned we can simplify geometry storing just storing vertex buffers and index buffer instead of storing separated triangles. And the reason because storing vertex buffers instead of triangle buffers is simpler is because triangles can share vertices so some vertices can be redundant if you dont re-use it.