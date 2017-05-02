extern crate rugl;

fn main() {
    let rugl = rugl::init();
    let count = 100;

    let draw = rugl.draw()
        .vert("
            #version 150
            in vec2 position;
            in float id;
            void main() {
                gl_Position = vec4(position * 0.05, 0.0, 1.0);
            }
        ")
        .frag("
            #version 150
            out vec4 out_color;
            void main() {
                out_color = vec4(1.0, 1.0, 1.0, 1.0);
            }
        ")
        .attribute("id", {
            &((0..count).map(|i| i as f32).collect::<Vec<f32>>())
        })
        .attribute("position", {
            &((0..(count * 3)).map(|i| {
                let id = (i as f32 / 3.0).floor() * 0.5;
                let x = 5.0 * id.cos() * id.log10();
                let y = 5.0 * id.sin() * id.log10();
                // Equilateral triangle
                match i % 3 {
                    0 => [x + 0.0, y + 0.5],
                    1 => [x + 0.36056, y + -0.5],
                    2 => [x + -0.36056, y + -0.5],
                    _ => panic!("Math is hard.")
                }
            }).collect::<Vec<[f32; 2]>>())
        })
        .count(count)
        .finalize();

    rugl.frame(|| {
        draw();
    });
}
