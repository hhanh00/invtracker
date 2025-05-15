use candle_core::{Result, Tensor};

pub struct Model {
    pub first: Tensor,
    pub second: Tensor,
}

impl Model {
    pub fn forward(&self, image: &Tensor) -> Result<Tensor> {
        let x = image.matmul(&self.first)?;
        let x = x.relu()?;
        x.matmul(&self.second)
    }
}
