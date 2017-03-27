#[allow(dead_code)]
#[allow(unused_imports)]

mod rugl {
    pub struct RuglConfig {
        pub vert: Option<&'static str>,
        pub frag: Option<&'static str>,
        pub int: i32
    }

    pub struct Rugl {
        pub config: RuglConfig
    }

    impl Rugl {
        pub fn new() -> Rugl {
            Rugl {
                config: RuglConfig {
                    vert: None,
                    frag: None,
                    int: 0
                }
            }
        }

        pub fn vert(mut self, source: &'static str) -> Rugl {
            self.config.vert = Some(source);
            self
        }

        pub fn output(&mut self) -> &mut Rugl {
            println!("yo");
            self
        }


        pub fn finalize(self) -> Box<Fn()> {
            let config = self.config;
            Box::new(move || {
                match config.vert {
                    Some(x) => println!("Some vert {:?}", x),
                    None => println!("No vert")
                };
            })
        }

    }
}

#[cfg(test)]
mod rugl_tests {
    use super::rugl::*;

    #[test]
    fn test_the_buider_pattern() {

        let draw = Rugl::new()
            .vert("foobar")
            .finalize();

        draw();
        draw();
        draw();
    }
}
