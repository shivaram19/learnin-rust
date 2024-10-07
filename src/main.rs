fn main(){
	let ans = is_even(20);
	println!("{}",ans);
}
fn is_even(num : i32) -> bool {
if num%2 == 0 {
return true;
}else{
return false;
}
}
//struct User {
//    single: bool,
//    username:String,
//    interests:String
//}
//
//struct Rect {
//    h:i3q,
//    w:i32
//}
//
//enum Direction {
//    North,
//    South,
//    East,
//    West
//}
//
//enum Shape{
//    circle(f64),
//    square(f64),
//    rectangle(f64, f64)
//}
//
//fn calculate_area(shape : Shape)-> f64 {
//     match shape{
//        Shape::circle(radius) => 3.14*radius*radius,
//        Shape::square(side_length)  => side_length*side_length,
//        Shape::rectangle(length,breadth ) => length*breadth
//    }
//}
//impl Rect {
//    fn area(&self)->i32{
//        return self.h*self.w;
//    }
//
//    fn perimeter(&self)->i32{
//        return 2*(self.h + self.w);
//    }
//}
//
//pub fn main(){
//    let user1 = User{
//        single:false,
//        username:String::from("Darling"),
//        interests:String::from("Badminton")
//    };
//
//    let rect = Rect{
//        h:3,
//        w:7
//    };
//    println!("the area of rectangle is {}",rect.area());
//    println!("the perimeter of rectangle is {}",rect.perimeter());
//    println!("username of userr1 is {:?}",user1.username);
//
//
//    // enums 
//    // let my_direction = Direction::East;
//    // println!("now i'm going in direction {}",my_direction);
//
//    // patternMatching
//    let circle = Shape::circle(5.0);
//    let square = Shape::square(40.0);
//    let rectangle = Shape::rectangle(70.0, 20.0);
//    println!("the area of the circle is {}",calculate_area(circle));
//} 
