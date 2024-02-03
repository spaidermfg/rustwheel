pub fn sort_thing() {
    println!("\n-----------------------------------sort thing");

    integer_sort();

    float_sort();

    struct_sort();
}

// 整数排序
fn integer_sort() {
    let mut v = vec![2, 45, 6, 3, 4, 6, 7, 89, 422, 34, 56];
    v.sort();
    println!("after sort: {:?}", v);
}

// 浮点数排序
fn float_sort() {
    let mut v = vec![4.5, 3.2, 6.0, 6.7, 1.3, 2.0, 1.1, 3.4, 5.6, 3.2, 2.3];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("after sort: {:?}", v);
}

// 结构体排序

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn struct_sort() {
    let p1 = Person::new("zack".to_string(), 23);
    let p2 = Person::new("mark".to_string(), 12);
    let p3 = Person::new("tom".to_string(), 78);

    // 自然排序
    let mut sort_p = vec![p1, p2, p3];
    sort_p.sort();
    println!("{:?}", sort_p);

    //按照age进行排序
    sort_p.sort_by(|a, b| b.age.cmp(&a.age));
    println!("{:?}", sort_p);
}
