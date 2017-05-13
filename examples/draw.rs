extern crate rugl;

fn main() {
    let mut rugl = rugl::init();
    let draw = {
        let mut builder = rugl.draw();
        builder.vert("
            #version 150
            in vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ")
        .frag("
            #version 150
            out vec4 out_color;
            void main() {
                out_color = vec4(1.0, 1.0, 1.0, 1.0);
            }
        ")
        .attribute("position", &vec![
             0.0f32,  0.5,
             0.5, -0.5,
            -0.5, -0.5
        ])
        .count(3);
        builder.finalize()
    };
    rugl.frame(|env| {
        draw(env);
    });
}
