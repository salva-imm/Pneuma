
#[derive(Debug)]
pub struct PneumaError {
    pub(crate) line: usize,
    pub(crate) message: String
}

impl PneumaError {
    fn new(&self, line: usize, message: String) -> Self {
        PneumaError {
            line,
            message
        }
    }
    pub(crate) fn report(&self) {
        eprintln!("~~~ line {} ~~~ \n Error {}", self.line, self.message);
    }
}