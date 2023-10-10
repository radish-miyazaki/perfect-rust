#![allow(dead_code)]

pub enum Shape {
    Rectangle { height: f64, width: f64 },
    Triangle { height: f64, bottom: f64 },
    Circle { radius: f64 },
    Trapezium { top: f64, bottom: f64, height: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Self::Rectangle { height, width } => height * width,
            Self::Triangle { height, bottom } => height * bottom / 2.0,
            Self::Circle { radius } => std::f64::consts::PI * radius * radius,
            Self::Trapezium { top, bottom, height } => (top + bottom) * height / 2.0,
        }
    }
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        match self {
            Self::Rectangle {..} => "This is a rectangle",
            Self::Triangle {..} => "This is a triangle",
            Self::Circle {..} => "This is a circle",
            Self::Trapezium {..} => "This is a trapezium",
        }.to_string()
    }
}

pub fn use_struct() {
    let rectangle = Shape::Rectangle { height: 10.0, width: 5.5 };
    let triangle = Shape::Triangle { height: 10.0, bottom: 5.0 };
    let circle = Shape::Circle { radius: 3.5 };
    let trapezium = Shape::Trapezium { bottom: 5.0, top: 3.0, height: 6.0 };

    println!("rectangle area: {}", rectangle.area());
    println!("triangle area: {}", triangle.area());
    println!("circle area: {}", circle.area());
    println!("trapezium area: {}", trapezium.area());
}

