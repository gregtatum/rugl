#[macro_use]
extern crate rugl;

fn main() {
    let mut rugl = rugl::init();
    let count = 10000;

    let draw = rugl.draw()
        .vert("
            #version 150
            in vec2 position;
            in float id;

            float ROTATION_SPEED = 0.5;
            float TRIANGLE_SIZE = 0.02;

            void main() {
                vec2 position2 = position + vec2(
                    cos(id * ROTATION_SPEED),
                    sin(id * ROTATION_SPEED)
                ) * sqrt(id);
                gl_Position = vec4(position2 * TRIANGLE_SIZE, 0.0, 1.0);
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
            &((0..count).map(|i| {
                (i as f32 / 3.0).floor()
            }).collect::<Vec<f32>>())
        })
        .attribute("position", {
            &((0..count).map(|i| {
                // Equilateral triangle
                match i % 3 {
                    0 => [0.0, 0.5],
                    1 => [0.36056, -0.5],
                    2 => [-0.36056, -0.5],
                    _ => panic!("Math is hard.")
                }
            }).collect::<Vec<[f32; 2]>>())
        })
        .count(count)
        .finalize();

    let clear = rugl_clear!(
        color => [0.3, 0.2, 0.3, 1.0],
        depth => 1.0
    );

    rugl.frame(|env| {
        clear();
        draw(env);
    });
}
