#[cfg(test)]
mod tests {
    use algebra::bw6_761::g1::{
            Parameters,
            G1_GENERATOR_X as Gx,
            G1_GENERATOR_Y as Gy,
        };
    use algebra_core::curves::{
        models::short_weierstrass_projective::GroupAffine,
        batch_arith::BatchGroupArithmetic
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn batch_add_double() {
        type Group = GroupAffine<Parameters>;
        
        let G = Group::new(Gx, Gy, false);
        
        let mut elems = [G+G, G+G+G]; 
        let mut elems2 = [G+G, G+G+G];
        let mut elems3 = [G+G, G+G+G];
        let mut elems4 = [G+G, G+G+G];
        BatchGroupArithmetic::batch_add_in_place(&mut elems, &mut elems2, &[(0,1)]);
        BatchGroupArithmetic::batch_add_in_place_same_slice(&mut elems3, &[(0,1)]);
        assert_eq!(elems, elems3);
        assert_eq!(elems2, elems4);
    }

}
