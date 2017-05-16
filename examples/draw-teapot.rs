mod models;
mod math;
use math::mat4;
use models::teapot;

extern crate rugl;

fn main() {
    let mut rugl = rugl::init();

    let draw = rugl.draw()
        .vert("
            #version 150
            in vec3 position;
            uniform mat4 projection;

            void main() {
                vec3 position2 = position * 0.05 + vec3(0.0, 0.0, -2.0);
                gl_Position = projection * vec4(position2, 1.0);
            }
        ")
        .frag("
            #version 150
            out vec4 out_color;
            void main() {
                out_color = vec4(1.0, 1.0, 1.0, 1.0);
            }
        ")
        .attribute("position", &teapot::POSITIONS)
        .elements(&teapot::CELLS)
        .uniform("projection", {
            let mut mat = mat4::identity();
            let mut width = 0;
            let mut height = 0;
            Box::new(move |env| {
                if env.viewport_width != width || env.viewport_height != height {
                    width = env.viewport_width;
                    height = env.viewport_height;
                    mat = mat4::perspective(1.0, 1.0, 0.1, 20.0);
                }
                Box::new(mat)
            })
        })
        .finalize();

    rugl.frame(|env| {
        draw(env);
    });
}
