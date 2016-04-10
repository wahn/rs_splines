/*!
# splines

To construct a `BezierCurve` you should use `BezierCurveBuilder`.

## Using **splines**

For a Bezier curve with a certain `degree` you need *degree+1*
**control vertices** (CVs).

```.rust
extern crate splines;
extern crate nalgebra as na;

use na::Vec4;

fn main() {
    // degree 2 curve (needs 3 CVs)
    let pt1 = Vec4::new(0.0, 0.0, 0.0, 1.0); // 0 0
    let pt2 = Vec4::new(1.0, 1.0, 0.0, 1.0); // 1 1
    let pt3 = Vec4::new(2.0, 0.0, 0.0, 1.0); // 2 0
    let curve = splines::BezierCurveBuilder::new()
        .add_cv(pt1)
        .add_cv(pt2)
        .add_cv(pt3)
        .finalize();
    println!("{:?}", curve);
    // degree 3 curve (needs 4 CVs)
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
}

```
*/

extern crate path;
extern crate nalgebra as na;

use na::Vec4;

#[derive(Debug)]
pub struct BezierCurve {
    /// control vertices (cvs)
    cvs: Vec<Vec4<f32>>,
    degree: usize,
    interval: (f32, f32),
}

impl BezierCurve {
    pub fn create_path(&self, acc: f32) -> path::Path {
        let mut pb = path::PathBuilder::new();
        let param = self.interval.0;
        let point = self.cvs[0];
        pb.add_sorted_point(point, param);
        self.length(acc, &mut pb);
        let path = pb.finalize();
        path
    }

    pub fn evaluate(&self) -> Vec<Vec4<f32>> {
        // TODO: calculate points on curve
        let mut pts: Vec<Vec4<f32>> = Vec::new();
        for n in 0..self.cvs.len() {
            pts.push(self.cvs[n]);
        }
        pts
    }
    /// Approximates the length of a Bezier curve within a given accuracy.
    pub fn length(&self, acc: f32, pb: &mut path::PathBuilder) -> f32 {
        // TODO: delegate length calculation to path crate
        let mut length: f32 = 0.0;
        // length of control point polyline
        for i in 1..self.cvs.len() {
            let vector: Vec4<f32> = self.cvs[i] - self.cvs[i-1];
            length += na::norm(&vector);
        }
        // distance between first and last control point
        let vector: Vec4<f32> = self.cvs[self.cvs.len()-1] - self.cvs[0];
        let start_end = na::norm(&vector);
        if (length - start_end) < acc {
            let param = self.interval.1;
            let cv = self.cvs[self.cvs.len() - 1];
            let point = Vec4::new(cv.x, cv.y, cv.z, cv.w);
            pb.add_sorted_point(point, param);
            length = start_end;
        } else {
            let (left, right) = self.split(0.5);
            length = left.length(acc, pb) + right.length(acc, pb);
        }
        length
    }
    /// Splits an existing Bezier curve into two parts at parameter value t.
    pub fn split(&self, t: f32) -> (BezierCurve, BezierCurve) {
        let mut pts: Vec<Vec4<f32>> = Vec::new();
        for n in 0..self.cvs.len() {
            pts.push(self.cvs[n]);
        }
        let mut indices: Vec<usize> = Vec::new();
        for n in 0..self.degree {
            indices.push(n+1);
        }
        for _i in 0..self.degree {
            for j in 0..indices.len() {
                let index = indices[j];
                let lerp = pts[index-1] * (1.0 - t) + pts[index] * t;
                pts.push(lerp);
            }
            let last_index = indices.pop().unwrap();
            for j in 0..indices.len() {
                indices[j] = last_index + 2 + j;
            }
        }
        let mut left_pts: Vec<Vec4<f32>> = Vec::new();
        let mut right_pts: Vec<Vec4<f32>> = Vec::new();
        let mut index1: usize = 0;
        let mut index2: usize = self.degree;
        let mut offset1: i8 = self.degree as i8 + 1;
        let mut offset2: i8 = self.degree as i8;
        for _i in 0..(self.degree + 1) {
            left_pts.push(pts[index1]);
            right_pts.push(pts[index2]);
            index1 = index1 + offset1 as usize;
            index2 = index2 + offset2 as usize;
            offset1 = offset1 - 1;
            offset2 = offset2 - 1;
        }
        right_pts.reverse();
        let ival: f32 = (1.0 - t) * self.interval.0 + t * self.interval.1;
        let mut left = BezierCurveBuilder::new();
        for point in left_pts {
            left.add_cv(point);
        }
        left.set_interval(self.interval.0, ival);
        let left = left.finalize();
        let mut right = BezierCurveBuilder::new();
        for point in right_pts {
            right.add_cv(point);
        }
        right.set_interval(ival, self.interval.1);
        let right = right.finalize();
        (left, right)
    }
}

/// Helper to construct a BezierCurve.

pub struct BezierCurveBuilder {
    cvs: Vec<Vec4<f32>>, // control vertices (cvs)
    interval: (f32, f32),
}

impl BezierCurveBuilder {
    /// Prepares the creation of a Bezier curve with an interval of [0,1].
    pub fn new() -> BezierCurveBuilder {
        BezierCurveBuilder { cvs: Vec::new(),
                             interval: (0.0f32, 1.0f32), }
    }
    /// Adds control vertices (CVs) to Bezier curve.
    pub fn add_cv(&mut self, cv: Vec4<f32>) -> &mut BezierCurveBuilder {
        self.cvs.push(cv);
        self
    }
    /// Optionally overwrite the default interval of [0,1].
    pub fn set_interval(&mut self, low: f32, high: f32) ->
        &mut BezierCurveBuilder {
        self.interval.0 = low;
        self.interval.1 = high;
        self
    }
    /// Number of CVs defines degree of Bezier curve.
    pub fn finalize(&self) -> BezierCurve {
        BezierCurve { cvs: self.cvs.to_vec(),
                      degree: self.cvs.len() - 1,
                      interval: self.interval, }
    }
}
