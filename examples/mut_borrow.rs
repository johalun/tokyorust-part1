fn main() {

   let mut x  = 4;
   if true {

      let y = &mut x;
      println!("Address of y is {:p}",y);
      *y = 1;
      }
      println!("Address of x is {:p}",&x);
}