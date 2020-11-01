#[cfg(test)]
mod tests {
    use algebra::bw6_761::{
        fq::*,
        fq3::*,
        g1::{Parameters, G1_GENERATOR_X as Gx, G1_GENERATOR_Y as Gy}};
    use algebra_core::{
        biginteger::{BigInteger, BigInteger768},
        fields::{
            FftField, FftParameters, Field, Fp12Parameters, Fp2Parameters, Fp6Parameters, FpParameters,
            SquareRootField,arithmetic,
        },
        curves::{
            batch_arith::BatchGroupArithmetic, models::short_weierstrass_projective::GroupAffine,
        },
        One, UniformRand, Zero,
    };
    
    use core::{
        cmp::Ordering,
        ops::{AddAssign, MulAssign, SubAssign},
    };

    use std::ptr;
    
  /*   use crate::bw6_761::{
        Fq, Fq3, Fq3Parameters, Fq6, Fq6Parameters, FqParameters, Fr
    }; */

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

    #[test]
    fn field_add_assign_fp768_test() {
        //define Field element (generator from G1)
        let a = Fq::new(BigInteger768([
            289919226011913130u64,
        13019990545710127566u64,
        4409829457611675068u64,
        13030600802816293865u64,
        15696054586628993047u64,
        9353078419867322391u64,
        5664203968291172875u64,
        5090703637405909511u64,
        17774776443174359288u64,
        10018561694451762270u64,
        12632664537138156478u64,
        46143195394855163u64,
        ]));
        let mut result = a;
        result.add_assign(Fq::new(BigInteger768([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])));
        //Assert if adding the neutral element to a field element returns the same element
        assert_eq!(result, a);
    }

    #[test]
    fn field_sub_assign_fp768_test() {
        //define Field element (generator from G1)
        let a = Fq::new(BigInteger768([
            289919226011913130u64,
        13019990545710127566u64,
        4409829457611675068u64,
        13030600802816293865u64,
        15696054586628993047u64,
        9353078419867322391u64,
        5664203968291172875u64,
        5090703637405909511u64,
        17774776443174359288u64,
        10018561694451762270u64,
        12632664537138156478u64,
        46143195394855163u64,
        ]));
        let mut result = a;
        result.sub_assign(Fq::new(BigInteger768([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])));
        //Assert if substracting the neutral element to a field element returns the same element
        assert_eq!(result, a);
    }

    #[test]
    fn field_mul_assign_fp768_test() {
        //define Field element (generator from G1)
        let a = Fq::new(BigInteger768([
            289919226011913130u64,
        13019990545710127566u64,
        4409829457611675068u64,
        13030600802816293865u64,
        15696054586628993047u64,
        9353078419867322391u64,
        5664203968291172875u64,
        5090703637405909511u64,
        17774776443174359288u64,
        10018561694451762270u64,
        12632664537138156478u64,
        46143195394855163u64,
        ]));
        let mut result = a;
        result.mul_assign(Fq::new(BigInteger768([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])));
        //Assert if mulitplying the neutral element to a field element returns the neutral element
        assert_eq!(result, Fq::new(BigInteger768([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])));
    }
    
/*     #[test]
    fn modsub768_test() {
        let mut a = ptr::null();
        #[cfg(use_bw6_asm)]
        #[allow(unsafe_code, unused_mut, unconditional_panic)]
        algebra_core::fields::arithmetic::modmul768(
            a,
            a,
            a,
            a,
        );
        assert_eq!(a, ptr::null());
    } */
}
