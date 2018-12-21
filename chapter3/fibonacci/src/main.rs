fn main() {
   let n = 10;
   let s = fibonacci(n);
   println!("The {}th fibonacci number is: {}", n, s);
}


fn fibonacci(n: u32) -> u32 {
   let mut f: u32 = 0;
   if n == 0{
      f = 0;
   } else if n == 1 {
     f = 1;
   } else {
     f = fibonacci(n-1) + fibonacci(n-2);
   }
   return f;
}