extern crate failure;

use std::io::Read;
use failure::Error;

pub struct State;

impl State {
    pub fn inject(&mut self, _reader: &mut Read) -> Result<(), Error> {
        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        State
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
