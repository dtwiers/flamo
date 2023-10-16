use num::Float;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Color<Scalar: Float> {
    pub red: Scalar,
    pub green: Scalar,
    pub blue: Scalar,
}

impl<Scalar: Float> Color<Scalar> {
    pub fn merge(&self, other: &Self) -> Self {
        let two = Scalar::one() + Scalar::one();
        Self {
            red: (self.red + other.red) / two,
            green: (self.green + other.green) / two,
            blue: (self.blue + other.blue) / two,
        }
    }
}
