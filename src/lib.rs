
#[derive(PartialEq, Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64
}

struct Scalar {
    value: f64
}

#[derive(PartialEq, Debug)]
struct Bivector<'a> {
    x: &'a Vector,
    y: &'a Vector
}

struct Trivector<'a> {
    x: &'a Vector,
    y: &'a Vector,
    z: &'a Vector
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector {
            x,
            y,
            z
        }
    }
}

impl<'a> Bivector<'a> {
    pub fn from_vectors(x: &'a Vector, y: &'a Vector) -> Self {
        Bivector {
            x,
            y
        }
    }
}

trait Magnitude {
    fn mag(&self) -> f64;
}

trait Angle {
    fn angle(&self, other: &Vector) -> f64;
}

trait InnerProduct {
    fn innerp(&self, other: &Vector) -> Scalar;
}

trait OuterProduct {
    fn outerp(&self, other: &Vector) -> Vector;
}

trait WedgeProduct<'a> {
    fn wedgep(&'a self, other: &'a Vector) -> Bivector;
}

trait GeometricProduct<'a> {
    fn geop(&'a self, other: &'a Vector) -> (Scalar, Bivector);
}

impl Magnitude for Vector {
    fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl<'a> Magnitude for Bivector<'a> {
    fn mag(&self) -> f64 {
        self.x.mag() * self.y.mag() * self.x.angle(&self.y).sin()
    }
}

impl Angle for Vector {
    fn angle(&self, other: &Vector) -> f64 {
        (self.innerp(other).value / (self.mag() * other.mag())).acos()
    }
}

impl InnerProduct for Vector {
    fn innerp(&self, other: &Vector) -> Scalar {
        Scalar {
            value: self.x * other.x + self.y * other.y + self.z * other.z
        }
    }
}

impl OuterProduct for Vector {
    fn outerp(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl<'a> WedgeProduct<'a> for Vector {
    fn wedgep(&'a self, other: &'a Vector) -> Bivector {
        Bivector {
            x: self,
            y: other
        }
    }    
}

impl<'a> GeometricProduct<'a> for Vector {
    fn geop(&'a self, other: &'a Vector) -> (Scalar, Bivector) {
        (self.innerp(other),
        Bivector {
            x: self,
            y: other
        })
    }    
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vector_mag() {
        let vec = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(1.0, vec.mag());
    }

    #[test]
    fn test_bivector_mag() {
        let vec1 = Vector::new(0.0, 1.0, 0.0);
        let vec2 = Vector::new(1.0, 0.0, 0.0);

        let bivec = Bivector::from_vectors(&vec2, &vec1);

        assert_eq!(1.0, bivec.mag());
    }

    #[test]
    fn test_vec_angle() {
        let vec1 = Vector::new(0.0, 1.0, 0.0);
        let vec2 = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(std::f64::consts::PI / 2.0, vec1.angle(&vec2));
    }

    #[test]
    fn test_innerp() {
        let vec1 = Vector::new(0.0, 1.0, 0.0);
        let vec2 = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(0.0, vec1.innerp(&vec2).value);
    }

    #[test]
    fn test_outerp() {
        let vec1 = Vector::new(0.0, 1.0, 0.0);
        let vec2 = Vector::new(1.0, 0.0, 0.0);

        let result_vec = Vector::new(0.0, 0.0, -1.0);

        assert_eq!(result_vec, vec1.outerp(&vec2));
    }

    #[test]
    fn test_wedgep() {
        let vec1 = Vector::new(0.0, 1.0, 0.0);
        let vec2 = Vector::new(1.0, 0.0, 0.0);

        let bivec: Bivector = vec1.wedgep(&vec2);

        assert_eq!(&vec1, bivec.x);
        assert_eq!(&vec2, bivec.y);
    }

    #[test]
    fn test_geop() {
        let vec1 = Vector::new(0.0, 1.0, 0.0);
        let vec2 = Vector::new(1.0, 0.0, 0.0);

        let sc: Scalar = vec1.innerp(&vec2);
        let bivec: Bivector = vec1.wedgep(&vec2);

        let geoprod: (Scalar, Bivector) = vec1.geop(&vec2);

        assert_eq!(sc.value, geoprod.0.value);
        assert_eq!(bivec, geoprod.1);
    }
}
