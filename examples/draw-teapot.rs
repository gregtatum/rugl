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
            uniform float time;

            void main() {
                gl_Position = vec4(
                    0.05 * position + vec3(time, 0.0, 0.0),
                    projection[0]
                );
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
        .uniform("time", Box::new(|env| {
            Box::new(env.time as f32)
        }))
        // .uniform("view", {
        //     // let eye = [0.0, 0.0, 20.0];
        //     // let center = [0.0, 0.0, 0.0];
        //     // let up = [0.0, 1.0, 0.0];
        //     // Box::new(move |env| Box::new({
        //     //     mat4::look_at(&eye, &center, &up)
        //     // }))
        //
        //     Box::new(move |env| Box::new(mat4::identity()))
        // })
        .uniform("projection", {
            Box::new(move |env| Box::new({
                let aspect = env.viewport_width as f32 / env.viewport_height as f32;
                let fovy = 1.0;
                mat4::perspective(fovy, aspect, 0.1, 100.0)
            }))
        })
        .finalize();

    rugl.frame(|env| {
        draw(env);
    });
}
