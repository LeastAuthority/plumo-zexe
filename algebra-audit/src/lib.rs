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
        type Group = GroupAffine<Parameters>;

        let G = Group::new(Gx, Gy, false);

        assert_eq!(2 + 2, 4);
    }

}
