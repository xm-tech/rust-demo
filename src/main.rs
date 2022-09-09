use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // 普通变量， 值不可以改变
    let num = 3;
    println!("num={},num again={0}", num);

    println!("{{}}");

    // 可变变量，值可以改变
    let mut num2 = 3.3;
    println!("num2={}", num2);
    num2 = 3.2;
    println!("num2={}", num2);

    const A: i32 = 1;
    println!("A={}", A);

    let x = 5;
    let x = x + 3;
    let x = x * 2;
    println!("x = {}", x);

    let y = "123";
    println!("y={}", y);

    let tup:(i32, f64, u8) = (500, 3.3, 8);
    println!("tup=({},{},{})", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];
    println!("a=[{},{},{},{},{}]", a[0], a[1], a[2], a[3], a[4]);

    let a: [i32; 5] = [5, 4, 3, 2, 1];
    println!("a=[{},{},{},{},{}]", a[0], a[1], a[2], a[3], a[4]);

    let a = [5; 10];
    println!("a={:?}", a);

    for e in a {
        print!("{},", e);
    }
    println!();

    // a[0] = 4; a 是不可变数组，不可赋值

    let afirst = a[0];
    let mut asecond = a[1];
    println!("afirst:{},asecond:{}", afirst, asecond);

    // 只有 mut 声明的可变数组才可重新赋值
    let mut a = [1,2,3];
    a[1] = -1;
    for e in a {
        print!("{},", e);
        println!();
    }

    // call another func

    asecond = 6;
    run(afirst, asecond);

    test_express();

    // inline func
    fn get_five() ->i32 {
        5 
    }
    println!("fu get_five returns : {:?}", get_five());


    let add = add(afirst, asecond);
    println!("add({},{})={}", afirst, asecond, add);

    let mut sex = 1;
    let mut sex_str = get_sex(sex);
    println!("sex = {}", sex_str);
    sex = 0;
    sex_str = get_sex(sex);
    println!("sex = {}", sex_str);

    print_sex(0);
    print_sex(1);

    test_while_loop();

    test_for_loop();

    test_hashmap();
}


// func no returns
fn run(a :i32, b :i32) {
    println!("run exec, a={}, b={}", a, b); 
}

fn test_express(){
    println!("run test_express");
    let x = 3;
    let y = {
        let x = 1;
        x + 1
    };

    println!("x = {}", x);
    println!("{:?}", y);
}

fn add(a :i32, b :i32) -> i32 {
    return a + b;
}

fn get_sex(s :i32) ->String{
    if s == 0 {
        return String::from("male");
    }
    return String::from("fmale");
}

fn print_sex(s :i32) {
    let sex = if s > 0 {String::from("fmale")} else {String::from("male")};
    println!("print_sex executed, sex={}", sex);
}

fn test_while_loop() {
    println!("test_while_loop begin");
    let mut n = 5;
    while n > 0 {
       n-=1; 
       println!("n={}", n);
    }
    println!("test_while_loop end")
}

fn test_for_loop() {
    println!("test_for_loop begin");
    let a = [1, 2, 3, 4, 5];
    let mut out :String = "".to_string();
    for e in a {
        out.push_str(e.to_string().as_str());   
        out.push_str(",");
    }
    out = out[0..out.len()-1].to_string();
    
    println!();

    println!("out={}", out);
    println!("test_for_loop end");
}

fn test_hashmap() {
    println!("test_hashmap begin");
    let mut map = HashMap::new();
    map.insert("age", "37");
    map.insert("sex", "male");

    map.entry("tall").or_insert("172");
    println!("age:{}", map.get("age").unwrap());
    println!("sex:{}", map.get("sex").unwrap());

    
    // will not insert, cause tall value already existed
    map.entry("tall").or_insert("175");

    for (k, v) in map.iter() {
        println!("k:{},v:{}", k, v);
    }
    println!("test_hashmap end");
}
