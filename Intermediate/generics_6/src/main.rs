// Generic function to swap two values
fn swap<T: Copy>(x: &mut T, y: &mut T) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

// Generic struct representing a point in 2D space
struct Point<T> {
    x: T,
    y: T,
}

// Generic enum representing an optional value
enum Option<T> {
    Some(T),
    None,
}

// Generic trait for calculating the distance
trait Distance<T> {
    fn distance(&self, other: &T) -> f64;
}

// Implementing the Distance trait for Point
impl<T> Distance<Point<T>> for Point<T>
where
    T: Copy + Into<f64>,
{
    fn distance(&self, other: &Point<T>) -> f64 {
        let dx = self.x.into() - other.x.into();
        let dy = self.y.into() - other.y.into();
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    // Using the generic swap function with integers
    let mut a = 10;
    let mut b = 20;
    swap(&mut a, &mut b);
    println!("a = {}, b = {}", a, b);

    // Using the generic swap function with strings
    let mut x = "hello";
    let mut y = "world";
    swap(&mut x, &mut y);
    println!("x = {}, y = {}", x, y);

    // Creating instances of generic Point struct
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.5, y: 2.5 };
    let p3 = Point { x: "a", y: "b" };

    // Using the generic Option enum
    let o1: Option<i32> = Option::Some(10);
    let o2: Option<f64> = Option::Some(3.14);
    let o3: Option<&str> = Option::Some("hello");
    let o4: Option<()> = Option::None;
}
