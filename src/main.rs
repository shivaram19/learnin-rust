// use  std::{fs::read_to_string, vec};
// use chrono::{ Local, Utc };

// pub fn hakuna() -> &Vec<i32>{
// 	let mut vec: Vec<i32> = Vec::new();
// 	for i in 0..10{
// 		vec.push(i);
// 	}

// 	return &mut vec;
// }
// pub fn filter_even(vec : &mut Vec<i32>) -> &Vec<i32>{
// 	let mut i:i32 = 0;
// 	while i < vec.len(){
// 		if vec[i]%2 == 1 {
// 			vec.remove(i);
// 		};
// 		i+=1;
// 	}

// 	return vec ;
// }
// pub fn hakuna(vec : &mut Vec<i32>) -> &Vec<i32>{
// 	for i in 0..10{
// 		vec.push(i);
// 	}

// 	return vec;
// }
//  fn main(){

// 	let mut shit: Vec<i32> = Vec::new();
// 	let vector = hakuna(&mut shit);
// 	print!("{:?}", vector);

// 	let current_utc_time = Utc::now();
// 	let current_time = Local::now();
// 	println!("current time is {} and cuurent_utc_time is {}", current_time, current_utc_time);
// 	let result = read_to_string("hello.txt");
// 	match result {
// 		Ok(data) => println!("contents in the file are {:?}" ,data),
// 		Err(err) => println!("error caused while reading the file {:?}", err)
// 	}
//  }


use std::{collections::HashMap, vec};

fn reallocation(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
	let mut users: HashMap<String, i32> = HashMap::new();
	for (key, value ) in vec{
		users.insert(key, value);
	}

	return users;
}
fn main(){
	let mut hakuna: Vec<(String, i32)> = Vec::new();
	hakuna.push((String::from("hakuna"),23));
	hakuna.push((String::from("pushing for life"), 32));
	println!("{:?}",hakuna);
	let ans = reallocation(hakuna);
	println!("{:?}",ans);
	

	let vec = vec![1, 2, 3];
	let v1_iter = vec.iter();

	let sum: &i32 = &v1_iter.sum();
	println!("value of sum is {}",*sum);
	// for v in v1_iter{
	// 	println!("{:?}",v);
	// }

	let mut users : HashMap<String, i32> = HashMap::new();
	users.insert(String::from("hakuna"), 32);
	users.insert(String::from("kai raja kaii"), 91);
	let jang: Option<&i32> = users.get("kai raja kaii");
	// println!("{}", jang);
	match jang{
		Some(k) => println!("{}", k),
		None => println!("the value that you are looking for is not present")
	}
	let numbers : Vec<String> = vec![String::from("hakuna matata"), String::from("jang kun lee")];
	println!("{:?}",numbers);
	let mut vec = Vec::new();	
	for i in 0..100{
		vec.push(i);
	}
	filter_even(&mut vec);
	// println!("{:?}",ans);
	println!("{:?}",vec);
}

pub fn filter_even(v: &mut Vec<i32>){
	// let mut hakuna: Vec<i32> = Vec::new();
	// println!("{}",v.len());
	// for i in (0..v.len()).rev(){
	// 	if v[i]%2 == 0 {
	// 		v.remove(i);
	// 	}
	// }
	// // return hakuna;

	let mut i = 0;
	while i < v.len(){
		if v[i]%2 == 0 {
			v.remove(i);
		}else {
			i += 1;
		}
	}
}
































// use std::fs;
// struct Rect{
// 	height : i32,
// 	width : i32
// }

// enum Shape {
// 	Rectangle(f64, f64),
// 	Circle(f64),
// 	Square(f64)
// }

//  fn first_find_a(s:String) -> Option<i32> {
// 	for(index , char) in s.chars().enumerate(){
// 		if char == 'a' {
// 			return Some(index as i32);
// 		}
// 	}
// 	return None;
//  }


// impl Rect {
// 	fn area (&self) -> i32 {
// 		self.height*self.width
// 	}

// 	fn perimeter(&self, _num : i32) -> i32 {
// 		_num*(self.height + self.width)
// 	}

// 	fn debug() -> String{
// 		String::from("just a string")
// 	}
// }

// fn calculate_area(shape : Shape) -> f64 {
// 	match shape {
// 		Shape::Rectangle(a,b ) => a*b,
// 		Shape::Circle(r) => 3.14*r*r,
// 		Shape::Square(s) => s*s
// 	}
// }
// fn main(){
// 	let s = String::from("lingocha");
// 	let index = first_find_a(s);
// 	match index {
// 		Some(value) => println!("{} is the value", value),
// 		None => println!("a is not found")
// 	}
// 	let rectangle = Shape::Rectangle(2.0, 3.0);
// 	// let circle = Shape::Circle(2.0);
// 	// let square = Shape::Square(5.0);

// 	let rectangle_area = calculate_area(rectangle);
// 	let square_area = calculate_area(Shape::Square(10.0));
// 	let circle_area = calculate_area(Shape::Circle(7.0));
// 	println!("{} is the square area",square_area);
// 	println!("{} is the rectangle area",rectangle_area);
// 	println!("{} is the circle area",circle_area);

// 	let rect = Rect{
// 		height : 20,
// 		width : 20
// 	};

// 	let area = rect.area();
// 	let perimeter = rect.perimeter(2);

// 	println!("{} is area",area);
// 	println!("{} is the perimeter",perimeter);
// 	println!("{}",Rect::debug());
	
// 	let _ans = is_even(20);
// 	let fib_seq = fib_seq(22); 
// 	println!("{}",fib_seq);
// }

// fn is_even(num : i32) -> bool {
// if num%2 == 0 {
// return true;
// }else{
// return false;
// }
// }
// fn fib_seq(num : i32) -> i32 {
// 	if num ==1 || num == 2 {
// return num ;
// };
// return fib_seq(num-1) + fib_seq(num-2) ;
// }

// //struct User {
// //    single: bool,
// //    username:String,
// //    interests:String
// //}
// //
// //struct Rect {
// //    h:i3q,
// //    w:i32
// //}
// //
// //enum Direction {
// //    North,
// //    South,
// //    East,
// //    West
// //}
// //
// //enum Shape{
// //    circle(f64),
// //    square(f64),
// //    rectangle(f64, f64)
// //}
// //
// //fn calculate_area(shape : Shape)-> f64 {
// //     match shape{
// //        Shape::circle(radius) => 3.14*radius*radius,
// //        Shape::square(side_length)  => side_length*side_length,
// //        Shape::rectangle(length,breadth ) => length*breadth
// //    }
// //}
// //impl Rect {
// //    fn area(&self)->i32{
// //        return self.h*self.w;
// //    }
// //
// //    fn perimeter(&self)->i32{
// //        return 2*(self.h + self.w);
// //    }
// //}
// //
// //pub fn main(){
// //    let user1 = User{
// //        single:false,
// //        username:String::from("Darling"),
// //        interests:String::from("Badminton")
// //    };
// //
// //    let rect = Rect{
// //        h:3,
// //        w:7
// //    };
// //    println!("the area of rectangle is {}",rect.area());
// //    println!("the perimeter of rectangle is {}",rect.perimeter());
// //    println!("username of userr1 is {:?}",user1.username);
// //
// //
// //    // enums 
// //    // let my_direction = Direction::East;
// //    // println!("now i'm going in direction {}",my_direction);
// //
// //    // patternMatching
// //    let circle = Shape::circle(5.0);
// //    let square = Shape::square(40.0);
// //    let rectangle = Shape::rectangle(70.0, 20.0);
// //    println!("the area of the circle is {}",calculate_area(circle));
// //} 
