use nalgebra_glm::RealField;

pub fn radians<N>(degrees: N) -> N where N: RealField {
    degrees * N::pi() / nalgebra_glm::convert(180.0)
}
