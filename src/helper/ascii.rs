use crate::structs::geometry::Size;

pub fn render_ascii_art(color_buffer: &[u8], size: &Size) -> String {
    let ascii_chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let mut output = String::new();

    for y in 0..size.height {
        for x in 0..size.width {
            let i = (y * size.width + x) * 4;
            let red = color_buffer[i] as f32;
            let green = color_buffer[i + 1] as f32;
            let blue = color_buffer[i + 2] as f32;

            let luminance = 0.2126 * red + 0.7152 * green + 0.0722 * blue;
            let idx = ((luminance / 255.0) * (ascii_chars.len() - 1) as f32).round() as usize;
            let ch = ascii_chars[idx];

            output.push(ch);
        }
        output.push('\n');
    }

    output
}
