pub fn do_live<'a>(choose: i32, in1: &'a str, in2: &'a str) -> &'a str {
    unsafe{
    if choose == 1{
        in2
    }else{
        in1
    }
}
}

pub fn max_num<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
      &x
    } else {
      &y
    }
  }

pub fn max_num2(x: i32, y: i32) -> i32 {
    if x > y {
      x
    } else {
      y
    }
}