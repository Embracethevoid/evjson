use std::time::Instant;

mod evjson;

fn main() {
    // let a = Number::Float(1.2);
    // let b = String::from("我是你爸爸");
    // let c = Number::Integer(2);
    // let mut v: Vec<EVValue> = Vec::new();
    // v.push(EVValue::_number(a));
    // v.push(EVValue::_string(b));
    // v.push(EVValue::_number(c));
    // let mut sub_object = new();
    // sub_object.insert(String::from("aa"), EVValue::_string(String::from("123")));
    // let mut sub_object2 = new();
    // sub_object2.insert(String::from("b"), EVValue::_string(String::from("123")));
    // v.push(EVValue::_object(Box::new(sub_object)));
    // println!("{:?}", v);
    // let mut object = new();
    // object.insert(String::from("a"), EVValue::_array(v));
    // object.insert(String::from("b"), EVValue::_object(Box::new(sub_object2)));
    // let s = sub_object2.stringify(2);
    // let s = String::from("{\"a\" :   \"1\"  , \"cc\" : [-1.0, \"123\" , [{}]], \"b\":false}");
    // println!("{:?}", s.chars());
    // let s = "null".to_string();
    let now = Instant::now();

    // let mut timer = Timer::new().unwrap();

    // let start = SystemTime::now();
    let mut success = 0;
    let mut failure = 0;
    for _i in 0..100000 {
        match evjson::parse("                      \n\n\n\n\n\n\n\n\n\n\n{\"a\":{\"b\":{\"c\":1},\"x\":[\"y\",{\"z\":-1}]}}".to_string()) {
            // JSON::parse("{\"a\":{\"b\":[{\"c\":{}}]}}".to_string());
            Ok(_json) => {
                success += 1 ;
            // println!("{}",evjson::stringify_value(&_json,2,0));
            } //println!("{:?}", _json),
            Err(_e) => (failure += 1),   //println!("{:?}", e),
        }
    }
    let new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
    // println!("Wait {} ms...", interval.num_milliseconds());
    // let since_the_epoch = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");
    // println!("{:?}", since_the_epoch.subsec_millis());
    println!(
        "have {} cases succeeded , {} cases failed",
        success, failure
    );

    let cs: Vec<char> = "我是你爸爸".chars().collect();
    print!("{:?}", cs[2]);
}
