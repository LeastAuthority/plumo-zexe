#[cfg(test)]
mod tests {
    use algebra::bw6_761::g1::{
            Parameters,
            G1_GENERATOR_X as Gx,
            G1_GENERATOR_Y as Gy,
        };
    use algebra_core::curves::models::short_weierstrass_projective::GroupAffine;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn batch_add_double() {
        type Curve = GroupAffine<Parameters>;

        let G = Curve{
            infinity: false,
            x: Gx,
            y: Gy,
            _params: {},
        };

        assert_eq!(2 + 2, 4);
    }

}
