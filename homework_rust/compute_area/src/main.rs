
struct Rectangle {
    width: f32,
    height: f32,
}

#[allow(dead_code)]
impl Rectangle {
    fn new(wh: f32, ht: f32) -> Rectangle {
        Rectangle {
            width: wh,
            height: ht,
        }
    }
}

struct Circle {
    radius: f32,
}

#[allow(dead_code)]
impl Circle {
    fn new(rs: f32) -> Circle {
        Circle {
            radius: rs
        }
    }
}


struct Triangle {
    base: f32,
    height: f32,
}

#[allow(dead_code)]
impl Triangle {
    fn new(be: f32, ht: f32) -> Triangle {
        Triangle {
            base: be,
            height: ht,
        }
    }
}


trait ComputeArea {
    fn get_area(&self) -> f32;
}

impl ComputeArea for Rectangle {
    fn get_area(&self) -> f32 {
        self.width * self.height
    }
}

impl ComputeArea for Circle {
    fn get_area(&self) -> f32 {
        3.14 * self.radius.powf(2.0)
    }
}

impl ComputeArea for Triangle {
    fn get_area(&self) -> f32 {
        (self.base * self.height) / 2.0
    }
}


fn get_area<T: ComputeArea>(t: &T) -> f32 { t.get_area() }

fn main() {
    let rectangle = Rectangle { width: 3.0, height: 4.0 };
    println!("rectangle Area: {}", get_area(&rectangle));
    let circle = Circle { radius: 3.0};
    println!("Circle Area: {}", get_area(&circle));
    let triangle = Triangle { base: 3.0, height: 4.0 };
    println!("triangle Area: {}", get_area(&triangle));
}
