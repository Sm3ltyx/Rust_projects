fn main() {
    let default_circle = Circle {
        radius: 1.0,
        pi: 3.14
    };

    let circle1 = Circle {
        radius: 2.0,
        ..default_circle
    };

    let circle2 = new_circle(1.3);
    let circle3 = new_circle(3.0); //No integer allowed | TO FIX!

    default_circle.print();
    circle1.print();
    circle2.print();
    circle3.print();
    println!("Can circle1 hold circle2? {}", circle1.can_hold(&circle2));
    println!("Can circle1 hold circle3? {}", circle1.can_hold(&circle3));
}

struct Circle {
        pi: f64,
        radius: f64,
}

fn new_circle(radius: f64) -> Circle{
    Circle {
        radius,
        pi: 3.14,
    }
}

//Methods

impl Circle {
    fn diameter (&self) -> f64{
        self.radius*2.0
    }

    fn area (&self) -> f64{
        self.radius*self.radius*self.pi
    }

    fn circumference (&self) -> f64 {
        self.radius*self.pi*2.0
    }

    fn print (&self){
        println!("PI: {}\nRadius: {}\nDiameter: {}\nCircumference: {}\nArea: {}\n", self.pi, self.radius, self.diameter(), self.circumference(), self.area());
    }

    fn can_hold (&self, other: &Circle) -> bool {
        self.radius > other.radius
    }
}

