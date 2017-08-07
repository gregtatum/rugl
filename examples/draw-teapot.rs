mod models;
mod math;
use math::mat4;
use models::teapot;
use math::angle_normals;

extern crate rugl;

fn main() {
    let mut rugl = rugl::init();

    let draw = rugl.draw()
        .vert("
            #version 150
            in vec3 position;
            in vec3 normal;
            uniform mat4 projection, view, model;
            uniform float time;
            out vec3 vNormal;

            void main() {
                vec3 position2 = position;
                vNormal = normal;
                gl_Position = projection * view * model * vec4(position2, 1.0);
            }
        ")
        .frag("
            #version 150
            out vec4 out_color;
            in vec3 vNormal;
            void main() {
                vec3 normal = normalize(vNormal);
                vec3 color =
                    0.5 * vec3(1.0, 0.5, 0.5) * vec3(max(0.0, dot(normal, vec3(0.0, 1.0, 0.0)))) +
                    0.3 * vec3(0.5, 1.0, 0.5) * vec3(max(0.0, dot(normal, vec3(1.0, 0.0, 0.0)))) +
                    0.2 * vec3(0.5, 0.5, 1.0) * vec3(max(0.0, dot(normal, vec3(0.0, 0.0, 1.0))));
                out_color = vec4(color, 1.0);
            }
        ")
        .attribute("position", &teapot::POSITIONS)
        .attribute("normal", &angle_normals::compute(teapot::CELLS, teapot::POSITIONS))
        .elements(&teapot::CELLS)
        .uniform("time", Box::new(|env| {
            Box::new(env.time as f32)
        }))
        .uniform("model", {
            let identity = mat4::identity();
            Box::new(move |env| Box::new({
                mat4::rotate_y(&identity, env.time as f32)
            }))
        })
        .uniform("view", {
            let view = mat4::translate(&mat4::identity(), &[0.0, 0.0, -50.0]);
            Box::new(move |_| Box::new({
                view
            }))
        })
        .uniform("projection", {
            Box::new(move |env| Box::new({
                let aspect = env.viewport_width as f32 / env.viewport_height as f32;
                let fovy = 1.0;
                mat4::perspective(fovy, aspect, 0.1, 1000.0)
            }))
        })
        .finalize();

    rugl.frame(|env| {
        draw(env);
    });
}
