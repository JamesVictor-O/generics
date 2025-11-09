// generics in rust;
//  gennerics allow you writte code that works for many types without sacrificing performances or safety.

//  generic in struct 
#[derive(Debug)]
struct Points<T>{
    x:T,
    y:T
}

enum Option<T>{
    Some(T),
    None
}

#[derive(Debug)]
enum Registerd<T>{
    Yes(T),
    No
}


fn main() {

    let points= Points{
        x:20,
        y:39
    };

    let mut points2= Points{
        x:0.3,
        y:0.3
    };
    println!("{:?}", points);

   let is_registered= Registerd::Yes(String::from("2025-11-06"));
   let is_registered2=Registerd::Yes(32);
   print_register(is_registered);
   print_register(is_registered2);

//    println!("{:?}", is_registered)

}

fn print_register<T: std::fmt::Display>(reg: Registerd<T>){
      match reg {
       Registerd::Yes(date) => println!("{}", date),
       
       Registerd::No => println!("user not registerd")
   }
}
