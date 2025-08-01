#[macro_use]
extern crate approx; // for relative_eq!
extern crate nalgebra as na;

use na::{Isometry2, Vector2};
use ncollide2d_updated as ncollide2d;
use ncollide2d::query;
use ncollide2d::shape::{Ball, Cuboid};

fn main() {
    let cuboid = Cuboid::new(Vector2::new(1.0, 1.0));
    let ball = Ball::new(1.0);

    let cuboid_pos = na::one();
    let ball_pos_intersecting = Isometry2::new(Vector2::y(), na::zero());
    let ball_pos_disjoint = Isometry2::new(Vector2::y() * 3.0, na::zero());

    let dist_intersecting = query::distance(&ball_pos_intersecting, &ball, &cuboid_pos, &cuboid);
    let dist_disjoint = query::distance(&ball_pos_disjoint, &ball, &cuboid_pos, &cuboid);

    assert_eq!(dist_intersecting, 0.0);
    assert!(relative_eq!(dist_disjoint, 1.0, epsilon = 1.0e-7));
}
