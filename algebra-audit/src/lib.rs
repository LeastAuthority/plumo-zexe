#[cfg(test)]
mod tests {
    use algebra::bw6_761::g1::{Parameters, G1_GENERATOR_X as Gx, G1_GENERATOR_Y as Gy};
    use algebra_core::curves::{
        batch_arith::BatchGroupArithmetic, models::short_weierstrass_projective::GroupAffine,
    };

    #[test]
    fn batch_add_in_place_happy_case() {
        type Group = GroupAffine<Parameters>;

        let g = Group::new(Gx, Gy, false);

        // these are all [[2]g, [3]g]
        let mut bases = [g + g, g + g + g];
        let mut others = [g + g, g + g + g];
        let mut bases_same_slice = [g + g, g + g + g];

        BatchGroupArithmetic::batch_add_in_place(&mut bases, &mut others, &[(0, 1)]);
        BatchGroupArithmetic::batch_add_in_place_same_slice(&mut bases_same_slice, &[(0, 1)]);

        // computation yields same result, with or without same_slice
        assert_eq!(bases, bases_same_slice);
    }

    #[test]
    fn batch_add_more_than_once_same_slice() {
        type Group = GroupAffine<Parameters>;
        let g = Group::new(Gx, Gy, false);

        let mut group_elem_bases_steps = [g + g, g + g + g];

        // BatchGroupArithmetic::batch_add_in_place_same_slice(&mut group_elem_bases, &[(0,0),(0,1)]);
        BatchGroupArithmetic::batch_add_in_place_same_slice(&mut group_elem_bases_steps, &[(1, 0)]);
        BatchGroupArithmetic::batch_add_in_place_same_slice(&mut group_elem_bases_steps, &[(1, 1)]);
        assert_eq!(group_elem_bases_steps[1], g + g + g + g + g + g + g);
        //   assert_eq!(group_elem_bases[0], g+g+g+g+g+g+g);
    }

    #[test]
    fn batch_add_in_place_reuse_bases_0() {
        type Group = GroupAffine<Parameters>;

        let g = Group::new(Gx, Gy, false);

        let mut group_elem_bases = [g + g, g + g + g];
        let mut group_elem_bases_steps = [g + g, g + g + g];

        // made several copies because at some point they say that the
        // second parameter to the function becomes a junk value after
        // the function was called.
        let mut group_elem_others = [g + g, g];
        let mut group_elem_others_steps1 = [g + g, g];
        let mut group_elem_others_steps2 = [g + g, g];

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
        assert_eq!(group_elem_bases_steps[0], g + g + g + g + g);

        // This one fails! This means that an index list that contains two elements
        // (x1, y1), (x2, y2), x1==x2 causes trouble.
        assert_eq!(group_elem_bases[0], g + g + g + g + g);
    }

    #[test]
    fn batch_add_in_place_reuse_bases_1() {
        type Group = GroupAffine<Parameters>;

        let g = Group::new(Gx, Gy, false);

        let mut group_elem_bases = [g + g, g + g + g];
        let mut group_elem_bases_steps = [g + g, g + g + g];

        // made several copies because at some point they say that the
        // second parameter to the function becomes a junk value after
        // the function was called.
        let mut group_elem_others = [g + g, g];
        let mut group_elem_others_steps1 = [g + g, g];
        let mut group_elem_others_steps2 = [g + g, g];

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
        assert_eq!(group_elem_bases_steps[1], g + g + g + g + g + g);

        // This one fails! This means that an index list that contains two elements
        // (x1, y1), (x2, y2), x1==x2 causes trouble.
        assert_eq!(group_elem_bases[1], g + g + g + g + g + g);
    }

    #[test]
    fn batch_add_in_place_reuse_others() {
        type Group = GroupAffine<Parameters>;

        let g = Group::new(Gx, Gy, false);

        let mut group_elem_bases = [g + g, g + g + g];
        let mut group_elem_bases_steps = [g + g, g + g + g];

        // made several copies because at some point they say that the
        // second parameter to the function becomes a junk value after
        // the function was called.
        let mut group_elem_others = [g + g, g];
        let mut group_elem_others_steps1 = [g + g, g];
        let mut group_elem_others_steps2 = [g + g, g];

        BatchGroupArithmetic::batch_add_in_place(
            &mut group_elem_bases,
            &mut group_elem_others,
            &[(0, 0), (1, 0)],
        );
        BatchGroupArithmetic::batch_add_in_place(
            &mut group_elem_bases_steps,
            &mut group_elem_others_steps1,
            &[(0, 0)],
        );
        BatchGroupArithmetic::batch_add_in_place(
            &mut group_elem_bases_steps,
            &mut group_elem_others_steps2,
            &[(1, 0)],
        );

        // Asserts if one call of batch_add_in_place with multiple indices results in the same
        // thing as two successive function calls with different indices.
        assert_eq!(group_elem_bases_steps, [g + g + g + g, g + g + g + g + g]);

        // This one fails! This means that an index list that contains two elements
        // (x1, y1), (x2, y2), y1==y2 causes trouble.
        assert_eq!(group_elem_bases, [g + g + g + g, g + g + g + g + g]);
    }

    #[test]
    fn bucketed_add_panic_1() {
        use algebra_core::curves::bucketed_add::{batch_bucketed_add, BucketPosition};

        type Group = GroupAffine<Parameters>;

        let g = Group::new(Gx, Gy, false);

        let buckets = 1;

        // these are all [[2]g, [3]g]
        let elems = [g + g, g + g + g];

        // panic on single bucket position
        let mut bucket_positions = [BucketPosition {
            bucket: 0,
            position: 0,
        }];

        batch_bucketed_add(buckets, &elems, &mut bucket_positions);
    }

    #[test]
    fn bucketed_add_panic_2() {
        use algebra_core::{
            curves::bucketed_add::{batch_bucketed_add, BucketPosition},
        };

        let g = GroupAffine::<Parameters>::new(Gx, Gy, false);

        let buckets = 2;

        // these are all [[1]g, [2]g, [3]g, [4]g]
        let elems = [g, g + g, g + g + g, g + g + g + g];

        // panic on position reuse
        let mut bucket_positions = [
            BucketPosition {
                bucket: 0,
                position: 0,
            },
            BucketPosition {
                bucket: 1,
                position: 0,
            },
        ];

        batch_bucketed_add(buckets, &elems, &mut bucket_positions);
    }

    #[test]
    fn bucketed_add_works() {
        use algebra_core::{
            curves::bucketed_add::{batch_bucketed_add, BucketPosition},
        };

        let g = GroupAffine::<Parameters>::new(Gx, Gy, false);

        let buckets = 2;

        // these are all [[1]g, [2]g, [3]g, [4]g]
        let elems = [g, g + g, g + g + g, g + g + g + g];

        // panic on position reuse
        let mut bucket_positions = [
            BucketPosition {
                bucket: 0,
                position: 0,
            },
            BucketPosition {
                bucket: 0,
                position: 0,
            },
            BucketPosition {
                bucket: 1,
                position: 0,
            },
            BucketPosition {
                bucket: 1,
                position: 1,
            },
        ];

        let res = batch_bucketed_add(buckets, &elems, &mut bucket_positions);

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], g+g);   // elems[0] + elems[0]
        assert_eq!(res[1], g+g+g); // elems[0] + elems[1]
    }
}
