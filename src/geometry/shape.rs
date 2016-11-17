use geometry::primitive::Point;
use linalg::{Dot, VectorNorm, Scalar, Vect};
use num::traits::Float;
use typehack::dim::Dim;


/// The `Shape` trait is fairly self-explanatory - it represents an n-dimensional shape, which is
/// assumed to have a centroid. `Shape`s have dimensionality (the `Dims` associated type) and also
/// an associated scalar type. We assume here that `Shape`s are dense enough to return dense
/// `Point`s and `Vect`s - although in the future, if sparse `Vect`s and `Mat`s are implemented,
/// this may change and become polymorphic.
pub trait Shape {
    type Scalar: Scalar;
    type Dims: Dim;

    fn dims(&self) -> Self::Dims;

    fn centroid(&self) -> Point<Self::Scalar, Self::Dims>;
}


/// Represents a convex shape. This trait is required for shapes to be used with the GJK distance
/// algorithm and MPR collision detection algorithm, among other things.
pub trait Convex: Shape {
    /// Returns an arbitrary interior point. Useful for algorithms like GJK, which require a point
    /// on the interior of something to start iterating with.
    fn interior_point(&self) -> Point<Self::Scalar, Self::Dims> {
        self.centroid()
    }
}


/// A support mapping on a convex shape is guaranteed to return the furthest point on a
/// shape in a given direction (of course there may in practice be multiple points, in which case)
/// the support function need not pick and choose, or fail - simply returning a valid support is
/// fine.
pub trait SupportMapping: Convex {
    fn support(&self, &Vect<Self::Scalar, Self::Dims>) -> Point<Self::Scalar, Self::Dims>;
}


/// One of the simplest possible structures, a `Cuboid` is an axis-aligned rectangular prism (also
/// affectionately known as an "AABB", or "axis-aligned bounding box".) It is composed of two
/// points, called here as its "corners". The `Cuboid` maintains an invariant on its corners: it
/// expects that for every element in its corner points, the first corner's element is less than
/// the second corner's element.
/// TODO: A constructor which checks this invariant, and an unsafe constructor which does not.
pub struct Cuboid<T: Scalar, D: Dim> {
    dims: D,
    corners: [Point<T, D>; 2],
}


impl<T: Clone + Scalar, D: Dim> Shape for Cuboid<T, D> {
    type Scalar = T;
    type Dims = D;

    fn dims(&self) -> D {
        self.dims
    }

    fn centroid(&self) -> Point<T, D> {
        ((Vect::<T, D>::from(self.corners[0].clone()) +
          Vect::<T, D>::from(self.corners[1].clone())) / (T::one() + T::one()))
            .into() // TODO: This is such a hack.
    }
}


impl<T: Clone + Scalar, D: Dim> Convex for Cuboid<T, D> {}


impl<T: Clone + Scalar, D: Dim> SupportMapping for Cuboid<T, D> {
    // TODO: Once the corner-order invariant is supported, fix this support mapping function such
    // that it takes advantage of it.
    fn support(&self, dir: &Vect<T, D>) -> Point<T, D> {
        let mut out = Point::from_elem(self.dims, &T::zero());

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


/// The `Sphere` represents an n-sphere with a radius and center. All shapes have their own local
/// coordinate systems, so the center is stored as a point with respect to its local origin.
pub struct Sphere<T: Scalar, D: Dim> {
    dims: D,
    center: Point<T, D>,
    radius: T,
}


impl<T: Clone + Scalar, D: Dim> Shape for Sphere<T, D> {
    type Scalar = T;
    type Dims = D;

    fn dims(&self) -> D {
        self.dims
    }

    fn centroid(&self) -> Point<T, D> {
        self.center.clone()
    }
}


impl<T: Clone + Scalar, D: Dim> Convex for Sphere<T, D> {}


impl<T: Clone + Scalar + Float, D: Dim> SupportMapping for Sphere<T, D> {
    fn support(&self, dir: &Vect<T, D>) -> Point<T, D> {
        let norm = dir.norm();
        self.center.clone() + (dir / norm) * &self.radius
    }
}


/// The `Polygon` represents a convex n-polytope. Like the `Cuboid`, it too maintains an invariant
/// - its points must be *wound*, such that on iterating through all of them,
pub struct Polygon<T: Scalar, D: Dim> {
    dims: D,
    points: Vec<Point<T, D>>,
    centroid: Option<Point<T, D>>,
}


impl<T: Scalar, D: Dim> Shape for Polygon<T, D> {
    type Scalar = T;
    type Dims = D;

    fn dims(&self) -> D {
        self.dims
    }

    fn centroid(&self) -> Point<T, D> {
        unimplemented!();
    }
}


impl<T: Clone + Scalar + From<usize>, D: Dim> Convex for Polygon<T, D> {
    fn interior_point(&self) -> Point<T, D> {
        let sum: Vect<T, D> = self.points.iter().cloned().map(Vect::from).sum();
        Point::from(sum / T::from(self.points.len()))
    }
}


impl<T: Clone + Scalar + From<usize>, D: Dim> SupportMapping for Polygon<T, D> {
    fn support(&self, dir: &Vect<T, D>) -> Point<T, D> {
        assert!(self.points.len() > 2);

        let mut vects = self.points.iter().cloned().map(Vect::from);

        let mut max = {
            let pt: Vect<T, D> = vects.next().unwrap();
            let proj = (&pt).dot(dir);

            (proj, pt)
        };

        for vect in vects {
            let proj: T = (&vect).dot(dir);

            if proj > max.0 {
                max = (proj, vect);
            }
        }

        max.1.into()
    }
}


pub struct MinkowskiSum<'a,
                        'b,
                        T: Scalar + 'a + 'b,
                        D: Dim + 'a + 'b,
                        A: Shape<Scalar = T, Dims = D> + 'a,
                        B: Shape<Scalar = T, Dims = D> + 'b>
{
    dims: D,
    a: &'a A,
    b: &'b B,
}


impl<'a, 'b, T: Scalar, D: Dim, A: Shape<Scalar = T, Dims = D>, B: Shape<Scalar = T, Dims = D>> Shape for MinkowskiSum<'a, 'b, T, D, A, B> {
    type Scalar = T;
    type Dims = D;

    fn dims(&self) -> D {
        self.dims
    }


    fn centroid(&self) -> Point<T, D> {
        self.a.centroid() + self.b.centroid().into()
    }
}

impl<'a,
     'b,
     T: Scalar + 'a + 'b,
     D: Dim + 'a + 'b,
     A: Shape<Scalar = T, Dims = D> + 'a,
     B: Shape<Scalar = T, Dims = D> + 'b> Convex for MinkowskiSum<'a, 'b, T, D, A, B> {}


impl<'a, 'b, T: Clone + Scalar, D: Dim, A: Shape<Scalar = T, Dims = D> + SupportMapping, B: Shape<Scalar = T, Dims = D> + SupportMapping> SupportMapping for MinkowskiSum<'a, 'b, T, D, A, B> {
    fn support(&self, dir: &Vect<T, D>) -> Point<T, D> {
        self.a.support(dir) + self.b.support(dir).into()
    }
}


pub struct MinkowskiDifference<'a,
                               'b,
                               T: Scalar + 'a + 'b,
                               D: Dim + 'a + 'b,
                               A: Shape<Scalar = T, Dims = D> + 'a,
                               B: Shape<Scalar = T, Dims = D> + 'b>
{
    dims: D,
    a: &'a A,
    b: &'b B,
}


impl<'a,
     'b,
     T: Scalar + 'a + 'b,
     D: Dim + 'a + 'b,
     A: Shape<Scalar = T, Dims = D> + 'a,
     B: Shape<Scalar = T, Dims = D> + 'b> Convex for MinkowskiDifference<'a, 'b, T, D, A, B> {}


impl<'a, 'b, T: Scalar, D: Dim, A: Shape<Scalar = T, Dims = D>, B: Shape<Scalar = T, Dims = D>> Shape for MinkowskiDifference<'a, 'b, T, D, A, B> {
    type Scalar = T;
    type Dims = D;

    fn dims(&self) -> D {
        self.dims
    }


    fn centroid(&self) -> Point<T, D> {
        self.a.centroid() - Vect::from(self.b.centroid())
    }
}


impl<'a, 'b, T: Clone + Scalar, D: Dim, A: Shape<Scalar = T, Dims = D> + SupportMapping, B: Shape<Scalar = T, Dims = D> + SupportMapping> SupportMapping for MinkowskiDifference<'a, 'b, T, D, A, B> {
    fn support(&self, dir: &Vect<T, D>) -> Point<T, D> {
        self.a.support(dir) - Vect::from(self.b.support(&-dir))
    }
}
