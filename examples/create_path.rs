extern crate splines;
extern crate nalgebra as na;

use na::Vec4;

fn main() {
    // degree 2 curve
    let pt1 = Vec4::new(0.0, 0.0, 0.0, 1.0); // 0 0
    let pt2 = Vec4::new(1.0, 1.0, 0.0, 1.0); // 1 1
    let pt3 = Vec4::new(2.0, 0.0, 0.0, 1.0); // 2 0
    let curve = splines::BezierCurveBuilder::new()
        .add_cv(pt1)
        .add_cv(pt2)
        .add_cv(pt3)
        .finalize();
    println!("{:?}", curve);
    let acc: f32 = 0.01;
    let path = curve.create_path(acc);
    println!("create_path({}) = {:?}", acc, path);
    let acc: f32 = 0.001;
    let path = curve.create_path(acc);
    println!("create_path({}) = {:?}", acc, path);
    // degree 3 curve
    let pt1 = Vec4::new(0.0, 0.0, 0.0, 1.0); // 0 0
    let pt2 = Vec4::new(0.0, 1.0, 0.0, 1.0); // 0 1
    let pt3 = Vec4::new(1.0, 1.0, 0.0, 1.0); // 1 1
    let pt4 = Vec4::new(1.0, 0.0, 0.0, 1.0); // 1 0
    let curve = splines::BezierCurveBuilder::new()
        .add_cv(pt1)
        .add_cv(pt2)
        .add_cv(pt3)
        .add_cv(pt4)
        .finalize();
    println!("{:?}", curve);
    let acc: f32 = 0.01;
    let path = curve.create_path(acc);
    println!("create_path({}) = {:?}", acc, path);
    let acc: f32 = 0.001;
    let path = curve.create_path(acc);
    println!("create_path({}) = {:?}", acc, path);
}
