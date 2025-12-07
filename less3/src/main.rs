use std::collections::HashMap;
fn main() {
    // Collections
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    v.push(1);
    v.push(10);
    v.push(12); // these will give error if you dont make v as mut
    println!("{:?}", v); // debug mode
    println!("{:?}", v1);

    // Indexing
    // Way #1
    println!("{}", v[1]);

    let second: &i32 = &v[1];
    // let second:i32=v[1]; what's the diff b/w to, mind both gives the same result
    println!("{second}");
    // println!("{}",v.get(1)); // this wont work

    // Way #2
    let third: Option<&i32> = v.get(1);
    match third {
        Some(third) => println!("{}", third),
        None => println!("Data not present"),
    }

    // just to print
    for i in &v {
        print!("{i} ");
    }
    println!();
    
    // to do something
    for i in &mut v {
        *i += 1;
        print!("{i} ");
    }
    println!();
    
    
    // String DS : feels hard for many rustaceans
    
    let s=String::new();
    println!("{:?}",s);

    let s1:Vec<char>=['s','t','r'].to_vec();
    // let s1:Vec<char>="string";
    println!("{:?}",s1);


    let mut s2=String::from("Foo");
    s2.push_str("bar");
    println!("{:?}",s2);

    // String in Rust does not allow indexing


    // HasMap
    let mut scores:HashMap<u8,i32>=HashMap::new();
    scores.insert(1,12);
    scores.insert(2,22);
    scores.insert(3,32);

    println!("{:?}",scores);

    let get_val_from_hash=1;
    // let score=scores.get(&get_val_from_hash).copied().unwrap_or(0);

    // println!("{score}");

    // Observation: This will work
    // for i in &scores{
    //     println!{"{:?}",i};
    // }
    // for (key,val) in scores{
    //     println!{"{key} : {val}"};
    // }
    // Observation: This wont work due to borrow rules
    // for i in scores{
    //     println!{"{:?}",i};
    // }
    // for (key,val) in scores{
    //     println!{"{key} : {val}"};
    // }
    // Observation: This will work
    // for i in &scores{
    //     println!{"{:?}",i};
    // }
    // for (key,val) in &scores{
    //     println!{"{key} : {val}"};
    // }

    for (key,_val) in &mut scores{
        // scores.entry(*key).or_insert(100);
        *_val+=100;
    }
    println!("{scores:?}");



}
