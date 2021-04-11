pub mod living;
use crate::living::{do_live, max_num, max_num2};

struct Option {
    pub some_int : i32,
    pub some_string : String
  }
  
  impl Option {
    pub fn new<T: Into<String>>(some_int : i32, some_string : T) -> Option {
      Option { some_int: some_int, some_string: some_string.into() }
    }
  }
  
  struct Service<'a> {
    option: &'a Option
  }
  
  impl<'a> Service<'a> {
    pub fn new(option: &'a Option) -> Service {
      Service { option: option }
    }
    
    pub fn name(&self) -> String {
      self.option.some_string.clone()
    }
  }


// fn create_service() -> Service {  // compile error
//     let opt = Option::new(1, "hello");
//     Service::new(&opt);
// }
#[derive(Debug)]
struct Foo<'a> {
  v: &'a i32
}



fn main() {
    // let foo;
    // {
    //     let v =123;
    //     foo = Foo {
    //         v: &v
    //     }
    // }
    // println!("{:?}", foo);

    let a = "2333";
    let b = "hhhh";
    let x = 1;
    let y = 9;

    let max; {
        max = max_num(&y, &x);
        // max = max_num2(x, y);
    }
    println!("max {}", max);
    let c = do_live(10, a, b);
    // let c = do_live2(10, a, b);
    // println!("{}", c);
    println!("{}", a);
    println!("{}", b);
    println!("Hello, world!");
    let opt = Option::new(1, "hello");
    let svc = Service::new(&opt);
    println!("Hello, {}", svc.name());
    // create_service();
}
