// generics in rust;
//  gennerics allow you writte code that works for many types without sacrificing performances or safety.

//  generic in struct 
#[derive(Debug)]
struct Points<T>{
    x:T,
    y:T
}

struct Person<T>{
    name:T,
    age:T,
    work:T
}

#[derive(Debug)]
enum Registerd<T>{
    Yes(T),
    No
}

impl<T>  Person<T> {
     fn new(name:T, age:T, work:T) ->Self{
        Self { name, age, work }
     }

     fn value_name(&self) -> &T{
        &self.name
     }
}

impl<T> Points<T> {
    fn new(x:T, y:T) -> Points<T>{
        Points{
            x,
            y
        }
    }

    fn value_x(&self) -> &T{
        &self.x
    }
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
