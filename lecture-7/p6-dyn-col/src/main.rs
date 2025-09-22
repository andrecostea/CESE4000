/*
 credit: https://teach-rs.trifectatech.org/exercises/2-foundations-of-rust/5-closures-and-dynamic-dispatch/index.html 
 */

trait Render {
    fn paint(&self);
}

struct Circle;
impl Render for Circle {
    fn paint(&self) { todo!() }
}

struct Rectangle;
impl Render for Rectangle {
    fn paint(&self) { todo!() }
}

/* 
 fn main() {
    let mut shapes = Vec::new();
    let circle = Circle;
    shapes.push(circle);
    let rect = Rectangle;
    shapes.push(rect);
    shapes.iter().for_each(|shape| shape.paint());
}
*/

fn main() {
    let mut shapes: Vec<Box<dyn Render>> = Vec::new();

    let circle = Box::new(Circle);
    shapes.push(circle);

    let rect = Box::new(Rectangle);
    shapes.push(rect);

    shapes.iter().for_each(|shape| shape.paint());
}