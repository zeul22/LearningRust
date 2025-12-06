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


    // String DS : feels hard for many rustaceans

}
