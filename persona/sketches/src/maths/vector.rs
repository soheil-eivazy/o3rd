
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {


    pub fn add(&mut self, other: Vector) {
        self.x =  self.x + other.x;
        self.y =  self.y + other.y;
    }

    pub fn mag(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.)).sqrt()
    }

    pub fn set_mag(&mut self, new_mag: f64) {
        let mag = self.mag();
        if mag == 0. {return}

        self.x = self.x * new_mag / mag;
        self.y = self.y * new_mag / mag;

        // println!("{:?}", self);
    }

    pub fn sub(&mut self, other: Vector) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }

    // pub fn mult(&mut self, scaler: i32) {
    //     self.x *= scaler;
    //     self.y *= scaler;
    // }

    // pub fn div(&mut self, scaler: i32) {
    //     self.x /= scaler;
    //     self.y /= scaler;
    // }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub start: Vector,
    pub end: Vector,
}


pub fn add_vectors(a: Vector, b: Vector) -> Vector {
    Vector { x: (a.x + a.x), y: (a.y + b.y) }
}


pub fn sub_vectors(a: Vector, b: Vector) -> Vector {
    Vector { x: (a.x - b.x), y: (a.y - b.y) }
}


pub fn mag_vectors(a: Vector, b: Vector) -> f64 {
    ((a.x + b.x).powf(2.) + (a.y + b.y).powf(2.)).sqrt()
}


pub fn mapping(number: f64, start1: f64, end1: f64, start2: f64, end2: f64) -> f64 {

    if number < start1 {
        return start2
    } else if number > end1 { 
        return end2
    }

    let res = (number * (end2 - start2)) / (end1 - start1);

    if end2 < start2 {
        start2 + end2 - res
    } else {
        res
    }
}
