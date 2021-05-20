use std::collections::HashMap;

pub enum Property {
    Clear,
    ClearStencil,
    ClearDepth,
    ClearColor,
}

pub struct State {
    pub changed: Vec<Property>
    pub clear_stencil: Option(GLEnum),
    pub clear_depth: Option(f64),
    pub clear_depth: Option(f64),
    pub clear_color: Option([f64; 4]),
}
