// generic struct
struct Point<T, U> {
    x : T,
    y : U
}

// generic struct's method
//we have to declare T just after impl so we can use it to specify that
// we’re implementing methods on the type Point<T>.
impl<T,U> Point<T, U> {
    fn get_x(&self) ->&T {
            &self.x
    }

    // use two or more generic type in method
    fn mixup<V, W>(self, p2: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: p2.y
        }
    }
}

fn main() {
    let p1 = Point {x: 1, y: 2.0};
    println!("p1's x={}", p1.get_x());

    let p3 = p1.mixup(Point {x: 1.0, y: 2});
    println!("mixup Point's x={}, y={}", p3.x, p3.y)
}
