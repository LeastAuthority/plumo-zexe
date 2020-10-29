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


    #[test]
    fn batch_add_more_than_once() {
        type Group = GroupAffine<Parameters>;

        let G = Group::new(Gx,Gy,false);

        let mut group_elem_1 = [G+G, G+G+G];
        let mut group_elem_2 = [G+G, G]; //made several copies because at some point they say that the second parameter to the function becomes a junk value after the function was called
        let mut group_elem_4 = [G+G, G];
        let mut group_elem_5 = [G+G, G];
        let mut group_elem_3 = [G+G, G+G+G];

        BatchGroupArithmetic::batch_add_in_place(&mut group_elem_1, &mut group_elem_2, &[(0,0), (0,1)]);
        BatchGroupArithmetic::batch_add_in_place(&mut group_elem_3, &mut group_elem_4, &[(0,0)]);
        BatchGroupArithmetic::batch_add_in_place(&mut group_elem_3, &mut group_elem_5, &[(0,1)]);

        // Asserts if one call of batch_add_in_place with multiple indices results in the same
        // thing as two successive function calls with different indices.
        assert_eq!(group_elem_3[0], G+G+G+G+G);
        assert_eq!(group_elem_1[0], G+G+G+G+G);
    }
    #[test]
    fn batch_add_more_than_once_second_index() {
        type Group = GroupAffine<Parameters>;

        let G = Group::new(Gx,Gy,false);

        let mut group_elem_1 = [G+G, G+G+G];
        let mut group_elem_2 = [G+G, G]; //made several copies because at some point they say that the second parameter to the function becomes a junk value after the function was called
        let mut group_elem_4 = [G+G, G];
        let mut group_elem_5 = [G+G, G];
        let mut group_elem_3 = [G+G, G+G+G];

        BatchGroupArithmetic::batch_add_in_place(&mut group_elem_1, &mut group_elem_2, &[(1,0), (1,1)]);
        BatchGroupArithmetic::batch_add_in_place(&mut group_elem_3, &mut group_elem_4, &[(1,0)]);
        BatchGroupArithmetic::batch_add_in_place(&mut group_elem_3, &mut group_elem_5, &[(1,1)]);

        // Asserts if one call of batch_add_in_place with multiple indices results in the same
        // thing as two successive function calls with different indices.
        assert_eq!(group_elem_3[1], G+G+G+G+G+G);
        assert_eq!(group_elem_1[1], G+G+G+G+G+G);
    }

}
