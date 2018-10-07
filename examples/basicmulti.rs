//! An example of using the `BasicMulti` noise function

extern crate noise;

use noise::BasicMulti;
use noise::utils::*;

fn main() {
    PlaneMapBuilder::new(&BasicMulti::default())
        .build()
        .write_to_file("basicmulti.png");
}
