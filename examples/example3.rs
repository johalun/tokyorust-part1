struct Rectangle{
    x : i32,
    y : i32
}

impl Rectangle{
    fn area(&self) -> i32{
        self.x*self.y
    }
}

fn main() {
    let rect = Rectangle{x:2,y:3};
    println!("Rectangle area is {}",rect.area());
}