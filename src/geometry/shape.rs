use linalg::{Dot, VectorNorm, Scalar, Vect};
use num::traits::Float;
use typehack::data::Size;


pub trait Shape {
    type Scalar: Scalar;
    type Dims: Size<Self::Scalar>;

    fn dims(&self) -> Self::Dims;
}


pub trait SupportMapping: Shape {
    fn support(&self, &Vect<Self::Scalar, Self::Dims>) -> Vect<Self::Scalar, Self::Dims>;
}


pub struct Cuboid<T: Scalar, D: Size<T>> {
    dims: D,
    corners: [Vect<T, D>; 2],
}


impl<T: Scalar, D: Size<T>> Shape for Cuboid<T, D> {
    type Scalar = T;
    type Dims = D;

    fn dims(&self) -> D {
        self.dims
    }
}


impl<T: Clone + Scalar, D: Size<T>> SupportMapping for Cuboid<T, D> {
    fn support(&self, dir: &Vect<T, D>) -> Vect<T, D> {
        let mut out = Vect::from_elem(self.dims, &T::zero());

        for i in 0..self.dims.reify() {
            if dir[i] < T::zero() {
                out[i] = if self.corners[0][i] < self.corners[1][i] {
                        &self.corners[0][i]
                    } else {
                        &self.corners[1][i]
                    }
                    .clone();
            } else {
                out[i] = if self.corners[0][i] < self.corners[1][i] {
                        &self.corners[1][i]
                    } else {
                        &self.corners[0][i]
                    }
                    .clone();
            }
        }

        out
    }
}


pub struct Sphere<T: Scalar, D: Size<T>> {
    dims: D,
    center: Vect<T, D>,
    radius: T,
}


impl<T: Scalar, D: Size<T>> Shape for Sphere<T, D> {
    type Scalar = T;
    type Dims = D;

    fn dims(&self) -> D {
        self.dims
    }
}


impl<T: Clone + Scalar + Float, D: Size<T>> SupportMapping for Sphere<T, D> {
    fn support(&self, dir: &Vect<T, D>) -> Vect<T, D> {
        let norm = dir.norm();
        &self.center + (dir / norm) * &self.radius
    }
}


pub struct Polygon<T: Scalar, D: Size<T>> {
    dims: D,
    points: Vec<Vect<T, D>>,
}


impl<T: Scalar, D: Size<T>> Shape for Polygon<T, D> {
    type Scalar = T;
    type Dims = D;

    fn dims(&self) -> D {
        self.dims
    }
}


impl<T: Clone + Scalar, D: Size<T>> SupportMapping for Polygon<T, D>
    where Vect<T, D>: Clone
{
    fn support(&self, dir: &Vect<T, D>) -> Vect<T, D> {
        assert!(self.points.len() > 2);

        let mut points = self.points.iter();

        let mut max = {
            let vect = points.next().unwrap();
            let proj = vect.dot(dir);

            (proj, vect)
        };

        for pt in points {
            let proj = pt.dot(dir);

            if proj > max.0 {
                max = (proj, pt);
            }
        }

        max.1.clone()
    }
}


pub struct MinkowskiSum<'a,
                        'b,
                        T: Scalar + 'a + 'b,
                        D: Size<T> + 'a + 'b,
                        A: Shape<Scalar = T, Dims = D> + 'a,
                        B: Shape<Scalar = T, Dims = D> + 'b>
{
    dims: D,
    a: &'a A,
    b: &'b B,
}


impl<'a, 'b, T: Scalar, D: Size<T>, A: Shape<Scalar = T, Dims = D>, B: Shape<Scalar = T, Dims = D>> Shape for MinkowskiSum<'a, 'b, T, D, A, B> {
    type Scalar = T;
    type Dims = D;

    fn dims(&self) -> D {
        self.dims
    }
}


impl<'a, 'b, T: Clone + Scalar, D: Size<T>, A: Shape<Scalar = T, Dims = D> + SupportMapping, B: Shape<Scalar = T, Dims = D> + SupportMapping> SupportMapping for MinkowskiSum<'a, 'b, T, D, A, B> {
    fn support(&self, dir: &Vect<T, D>) -> Vect<T, D> {
        self.a.support(dir) + self.b.support(dir)
    }
}


pub struct MinkowskiDifference<'a,
                               'b,
                               T: Scalar + 'a + 'b,
                               D: Size<T> + 'a + 'b,
                               A: Shape<Scalar = T, Dims = D> + 'a,
                               B: Shape<Scalar = T, Dims = D> + 'b>
{
    dims: D,
    a: &'a A,
    b: &'b B,
}


impl<'a, 'b, T: Scalar, D: Size<T>, A: Shape<Scalar = T, Dims = D>, B: Shape<Scalar = T, Dims = D>> Shape for MinkowskiDifference<'a, 'b, T, D, A, B> {
    type Scalar = T;
    type Dims = D;

    fn dims(&self) -> D {
        self.dims
    }
}


impl<'a, 'b, T: Clone + Scalar, D: Size<T>, A: Shape<Scalar = T, Dims = D> + SupportMapping, B: Shape<Scalar = T, Dims = D> + SupportMapping> SupportMapping for MinkowskiDifference<'a, 'b, T, D, A, B> {
    fn support(&self, dir: &Vect<T, D>) -> Vect<T, D> {
        self.a.support(dir) - self.b.support(&-dir)
    }
}
