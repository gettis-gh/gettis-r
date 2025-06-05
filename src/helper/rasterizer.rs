use crate::structs::geometry::{Size, Triangle3};

pub fn create_spatial_grid(triangles: &Vec<Triangle3>, frame_size: &Size, grid_size: &Size) -> Vec<Vec<Triangle3>> {
    let mut grid: Vec<Vec<Triangle3>> = vec![Vec::new(); (grid_size.width * grid_size.height) as usize];

    let cell_width = frame_size.width as f32 / grid_size.width as f32;
    let cell_height = frame_size.height as f32 / grid_size.height as f32;

    for tri in triangles {
        let bbox = tri.bounding_box();

        // Convierte las coordenadas del bounding box (en pixeles) a índices de celdas
        let left_cell = (bbox.left as f32 / cell_width).floor() as usize;
        let right_cell = (bbox.right as f32 / cell_width).floor() as usize;
        let bottom_cell = (bbox.bottom as f32 / cell_height).floor() as usize;
        let top_cell = (bbox.top as f32 / cell_height).floor() as usize;

        // Asegura que los índices estén dentro del rango válido de la grilla
        let left_cell = left_cell.min(grid_size.width - 1);
        let right_cell = right_cell.min(grid_size.width - 1);
        let bottom_cell = bottom_cell.min(grid_size.height - 1);
        let top_cell = top_cell.min(grid_size.height - 1);

        // Recorre las celdas del grid que intersectan el bounding box
        for grid_row in bottom_cell..=top_cell {
            for grid_col in left_cell..=right_cell {
                let grid_index = grid_row * grid_size.width + grid_col;
                grid[grid_index].push(tri.clone());
            }
        }
    }

    grid
}