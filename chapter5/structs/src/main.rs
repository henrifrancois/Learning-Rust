#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
       self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
       self.width > rect.width && self.height > rect.height
    }
}

fn main() {
   let rect1 = Rectangle{ width:30, height:50};
   let rect2 = Rectangle{ width: 60, height: 35};
   println!("rectangle 1 area: {}px", rect1.area());
   println!("rectangle 2 area: {}px", rect2.area());
   if rect1.can_hold(&rect2) {
      println!("rect1 can hold rect2");
   } else {
      println!("rect1 cannot hold rect2");
   }
}
