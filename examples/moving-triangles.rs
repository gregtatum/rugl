extern crate rugl;

fn main() {
    let mut rugl = rugl::init();
    let count = 1000;

    let draw = rugl.draw()
        .vert("
            #version 150
            in vec2 position;
            in float id;

            uniform float time;
            uniform float count;

            float TRIANGLE_SIZE = 0.02;

            void main() {
                float unit_id = id / count;
                vec2 offset = vec2(
                    2.0 * (unit_id - 0.5) + 0.1 * sin(unit_id * 30.0 + time * 2.5) + 0.5 * sin(unit_id * 50.0 + time * 5.234),
                    (sin(unit_id * 20.0 + time) + sin(unit_id * 7.0 + time * 2.83)) / 3.0 + 0.5 * sin(unit_id * 66.0 + time * 7.234)
                );
                gl_Position = vec4(
                    position * TRIANGLE_SIZE + offset,
                    0.0,
                    1.0
                );
            }
        ")
        .frag("
            #version 150
            out vec4 out_color;
            uniform vec3 color;
            uniform vec3 color2;
            uniform vec3 color3;
            void main() {
                out_color = vec4(color + color2 + color3, 1.0);
            }
        ")
        .uniform("color", Box::new(|_| Box::new([1.0, 0.0, 0.0])))
        .uniform("color2", Box::new(|_| Box::new([0.0, 1.0, 0.0])))
        .uniform("color3", Box::new(|_| Box::new([0.0, 0.0, 1.0])))
        .uniform("time", Box::new(|env| Box::new(env.time as f32)))
        .uniform("count", {
            let count_in = count.clone() as f32;
            Box::new(move |env| Box::new(count_in))
        })
        .attribute("position", {
            &((0..(count * 3)).map(|i| {
                // Equilateral triangle
                match i % 3 {
                    0 => [0.0, 0.5],
                    1 => [0.36056, -0.5],
                    2 => [-0.36056, -0.5],
                    _ => panic!("Math is hard.")
                }
            }).collect::<Vec<[f32; 2]>>())
        })
        .attribute("id", {
            &((0..(count * 3)).map(|i| {
                (i as f32 / 3.0).floor()
            }).collect::<Vec<f32>>())
        })
        .count(count * 3)
        .finalize();

    rugl.frame(|env| {
        draw(env);
    });
}
