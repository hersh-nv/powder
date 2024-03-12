#[derive(Debug)]
pub struct ParameterError;

impl std::fmt::Display for ParameterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Couldn't make this parameter change")
    }
}

#[derive(Clone)]
pub struct Parameters {
    pub sandbox_w: i32,
    pub sandbox_h: i32,
}

impl Parameters {
    pub fn new(sandbox_size: i32) -> Self {
        let sandbox_w = sandbox_size;
        let sandbox_h = sandbox_size;
        // for now things are gonna break in the renderer if this isn't square
        // so let's just mandate that for now
        assert_eq!(sandbox_w, sandbox_h);

        Parameters {
            sandbox_w: sandbox_w,
            sandbox_h: sandbox_h,
        }
    }
}
