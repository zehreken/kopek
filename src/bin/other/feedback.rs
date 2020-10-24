extern crate kopek;
use nannou::prelude::*;

mod other;

pub fn start() {
    nannou::app(model).run();
}

struct Model {}

fn model(app: &App) -> Model {
    Model {}
}
