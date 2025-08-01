extern crate nalgebra as na;

use na::{Isometry3, Vector3};
use ncollide3d_updated as ncollide3d;
use ncollide3d::query::{self, Proximity};
use ncollide3d::shape::{Ball, Cuboid};

fn main() {
    let cuboid = Cuboid::new(Vector3::new(1.0, 1.0, 1.0));
    let ball = Ball::new(1.0);
    let margin = 1.0;

    let cuboid_pos = na::one();
    let ball_pos_intersecting = Isometry3::new(Vector3::new(1.0, 1.0, 1.0), na::zero());
    let ball_pos_within_margin = Isometry3::new(Vector3::new(2.0, 2.0, 2.0), na::zero());
    let ball_pos_disjoint = Isometry3::new(Vector3::new(3.0, 3.0, 3.0), na::zero());

    let prox_intersecting =
        query::proximity(&ball_pos_intersecting, &ball, &cuboid_pos, &cuboid, margin);
    let prox_within_margin =
        query::proximity(&ball_pos_within_margin, &ball, &cuboid_pos, &cuboid, margin);
    let prox_disjoint = query::proximity(&ball_pos_disjoint, &ball, &cuboid_pos, &cuboid, margin);

    assert_eq!(prox_intersecting, Proximity::Intersecting);
    assert_eq!(prox_within_margin, Proximity::WithinMargin);
    assert_eq!(prox_disjoint, Proximity::Disjoint);
}
