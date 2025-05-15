use candle_core::{Device, Tensor};
use flutter_rust_bridge::frb;

use crate::candle_test::Model;

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[frb]
pub fn candle_test() -> anyhow::Result<String> {
    let device = Device::Cpu;

    let first = Tensor::randn(0f32, 1.0, (784, 100), &device)?;
    let second = Tensor::randn(0f32, 1.0, (100, 10), &device)?;
    let model = Model { first, second };

    let dummy_image = Tensor::randn(0f32, 1.0, (1, 784), &device)?;

    let digit = model.forward(&dummy_image)?;
    let res = format!("Digit {digit:?} digit");

    Ok(res)
}

