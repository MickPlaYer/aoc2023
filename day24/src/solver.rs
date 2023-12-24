use crate::structs::Hailstone;
use num_bigint::BigInt;
use std::ops::{Add, Mul};
use z3::{
    ast::{Ast, Int, Real},
    Config, Context, SatResult, Solver,
};

pub(crate) fn solve(hailstones: Vec<Hailstone>) -> isize {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = Real::new_const(&ctx, "x");
    let y = Real::new_const(&ctx, "y");
    let z = Real::new_const(&ctx, "z");
    let vx = Real::new_const(&ctx, "vx");
    let vy = Real::new_const(&ctx, "vy");
    let vz = Real::new_const(&ctx, "vz");
    let solver = Solver::new(&ctx);
    for (i, stone) in hailstones.iter().enumerate() {
        let t = Real::new_const(&ctx, format!("t{}", i));
        let zero = Real::from_int(&Int::from_u64(&ctx, 0));
        solver.assert(&t.ge(&zero));
        let stone_x = real_from_bigint(&ctx, &stone.position.x);
        let stone_vx = real_from_bigint(&ctx, &stone.velocity.x);
        let stone_y = real_from_bigint(&ctx, &stone.position.y);
        let stone_vy = real_from_bigint(&ctx, &stone.velocity.y);
        let stone_z = real_from_bigint(&ctx, &stone.position.z);
        let stone_vz = real_from_bigint(&ctx, &stone.velocity.z);
        assert_stone_line_hit_current_line(&solver, &x, &vx, &t, stone_vx, stone_x);
        assert_stone_line_hit_current_line(&solver, &y, &vy, &t, stone_vy, stone_y);
        assert_stone_line_hit_current_line(&solver, &z, &vz, &t, stone_vz, stone_z);
    }
    if let SatResult::Sat = solver.check() {
        let model = solver.get_model().unwrap();
        let x = model.eval(&x, true).unwrap().as_real().unwrap();
        let y = model.eval(&y, true).unwrap().as_real().unwrap();
        let z = model.eval(&z, true).unwrap().as_real().unwrap();
        return ((x.0 / x.1) + (y.0 / y.1) + (z.0 / z.1)) as isize;
    } else {
        panic!("?????")
    }
}

fn assert_stone_line_hit_current_line(
    solver: &Solver<'_>,
    x: &Real<'_>,
    vx: &Real<'_>,
    t: &Real<'_>,
    stone_vx: Real<'_>,
    stone_x: Real<'_>,
) {
    solver.assert(
        &x.clone()
            .add(&vx.clone().mul(&t.clone()))
            ._eq(&t.clone().mul(&stone_vx).add(&stone_x)),
    );
}

fn real_from_bigint<'a>(ctx: &'a Context, big: &BigInt) -> Real<'a> {
    Real::from_int(&Int::from_str(ctx, &format!("{}", big)).unwrap())
}
