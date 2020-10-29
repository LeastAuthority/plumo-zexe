#[cfg(test)]
mod tests {
    use algebra::bw6_761::g1::{Parameters, G1_GENERATOR_X as Gx, G1_GENERATOR_Y as Gy};
    use algebra_core::curves::{
        batch_arith::BatchGroupArithmetic, models::short_weierstrass_projective::GroupAffine,
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn batch_add_double() {
        type Group = GroupAffine<Parameters>;

        let G = Group::new(Gx, Gy, false);

        let mut elems = [G + G, G + G + G];
        let mut elems2 = [G + G, G + G + G];
        let mut elems3 = [G + G, G + G + G];
        let mut elems4 = [G + G, G + G + G];
        BatchGroupArithmetic::batch_add_in_place(&mut elems, &mut elems2, &[(0, 1)]);
        BatchGroupArithmetic::batch_add_in_place_same_slice(&mut elems3, &[(0, 1)]);
        assert_eq!(elems, elems3);
        assert_eq!(elems2, elems4);
    }

    #[test]
    fn batch_add_more_than_once() {
        type Group = GroupAffine<Parameters>;

        let G = Group::new(Gx, Gy, false);

        let mut group_elem_bases = [G + G, G + G + G];
        let mut group_elem_bases_steps = [G + G, G + G + G];

        // made several copies because at some point they say that the
        // second parameter to the function becomes a junk value after
        // the function was called.
        let mut group_elem_others = [G + G, G];
        let mut group_elem_others_steps1 = [G + G, G];
        let mut group_elem_others_steps2 = [G + G, G];

        BatchGroupArithmetic::batch_add_in_place(
            &mut group_elem_bases,
            &mut group_elem_others,
            &[(0, 0), (0, 1)],
        );
        BatchGroupArithmetic::batch_add_in_place(
            &mut group_elem_bases_steps,
            &mut group_elem_others_steps1,
            &[(0, 0)],
        );
        BatchGroupArithmetic::batch_add_in_place(
            &mut group_elem_bases_steps,
            &mut group_elem_others_steps2,
            &[(0, 1)],
        );

        // Asserts if one call of batch_add_in_place with multiple indices results in the same
        // thing as two successive function calls with different indices.
        assert_eq!(group_elem_bases_steps[0], G + G + G + G + G);

        // This one fails! This means that an index list that contains two elements
        // (x1, y1), (x2, y2), x1==x2 causes trouble.
        assert_eq!(group_elem_bases[0], G + G + G + G + G);
    }

    #[test]
    fn batch_add_more_than_once_second_index() {
        type Group = GroupAffine<Parameters>;

        let G = Group::new(Gx, Gy, false);

        let mut group_elem_bases = [G + G, G + G + G];
        let mut group_elem_bases_steps = [G + G, G + G + G];

        // made several copies because at some point they say that the
        // second parameter to the function becomes a junk value after
        // the function was called.
        let mut group_elem_others = [G + G, G];
        let mut group_elem_others_steps1 = [G + G, G];
        let mut group_elem_others_steps2 = [G + G, G];

        BatchGroupArithmetic::batch_add_in_place(
            &mut group_elem_bases,
            &mut group_elem_others,
            &[(1, 0), (1, 1)],
        );
        BatchGroupArithmetic::batch_add_in_place(
            &mut group_elem_bases_steps,
            &mut group_elem_others_steps1,
            &[(1, 0)],
        );
        BatchGroupArithmetic::batch_add_in_place(
            &mut group_elem_bases_steps,
            &mut group_elem_others_steps2,
            &[(1, 1)],
        );

        // Asserts if one call of batch_add_in_place with multiple indices results in the same
        // thing as two successive function calls with different indices.
        assert_eq!(group_elem_bases_steps[1], G + G + G + G + G + G);

        // This one fails! This means that an index list that contains two elements
        // (x1, y1), (x2, y2), x1==x2 causes trouble.
        assert_eq!(group_elem_bases[1], G + G + G + G + G + G);
    }
}
