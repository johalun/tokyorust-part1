fn main() {

   let mut x  = Box::new(4);
   println!("Value of x is {}",x);
   
   if true {

      let y = x;
      println!("Value of y is {}",y);
      x = y;
      }
      println!("Value of x is {}",x);
}