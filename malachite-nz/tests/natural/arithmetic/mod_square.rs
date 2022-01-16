use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModMul, ModNeg, ModSquare, ModSquareAssign, Square,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base_test_util::generators::unsigned_pair_gen_var_16;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::generators::{
    natural_gen_var_2, natural_pair_gen_var_8, natural_triple_gen_var_3,
};
use std::str::FromStr;

// TODO improve from_str

#[test]
fn test_mod_square() {
    let test = |u, m, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_is_reduced(&Natural::from_str(m).unwrap()));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_square_assign(Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&Natural::from_str(m).unwrap()));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_square_assign(&Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_square(Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_square(Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_square(&Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_square(&Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "1", "0");
    test("1", "10", "1");
    test("2", "10", "4");
    test("100", "497", "60");
    test("200", "497", "240");
    test("300", "497", "43");
    test("1234567890", "123456789876", "100296296172");
}

#[test]
fn mod_square_properties() {
    natural_pair_gen_var_8().test_properties(|(x, m)| {
        assert!(x.mod_is_reduced(&m));
        let square_val_val = x.clone().mod_square(m.clone());
        let square_ref_val = (&x).mod_square(m.clone());
        let square_val_ref = x.clone().mod_square(&m);
        let square = (&x).mod_square(&m);
        assert!(square_val_val.is_valid());
        assert!(square_ref_val.is_valid());
        assert!(square_val_ref.is_valid());
        assert!(square.is_valid());
        assert!(square.mod_is_reduced(&m));
        assert_eq!(square_val_val, square);
        assert_eq!(square_ref_val, square);
        assert_eq!(square_val_ref, square);

        let mut mut_x = x.clone();
        mut_x.mod_square_assign(m.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, square);
        let mut mut_x = x.clone();
        mut_x.mod_square_assign(&m);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, square);

        assert_eq!((&x).mod_mul(&x, &m), square);
        assert_eq!((&x).square() % &m, square);
        assert_eq!(x.mod_neg(&m).mod_square(&m), square);
    });

    natural_gen_var_2().test_properties(|m| {
        assert_eq!(Natural::ZERO.mod_square(&m), 0);
        if m != 1 {
            assert_eq!(Natural::ONE.mod_square(m), 1);
        }
    });

    natural_triple_gen_var_3().test_properties(|(x, y, m)| {
        assert_eq!(
            (&x).mod_mul(&y, &m).mod_square(&m),
            x.mod_square(&m).mod_mul(y.mod_square(&m), &m)
        );
    });

    unsigned_pair_gen_var_16::<Limb>().test_properties(|(x, m)| {
        assert_eq!(
            x.mod_square(m),
            Natural::from(x).mod_square(Natural::from(m))
        );
    });
}
