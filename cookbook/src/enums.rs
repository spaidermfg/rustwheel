pub fn enum_mind() {
    mind();
}


#[derive(Debug)]
enum System {
    Linux,
    Android{ vendor: String, date: f64}, // 类结构体枚举变体
    IOS(String, f64),   //枚举变体
}


impl System {
    pub fn use_android(&self) -> System {
        System::Android{vendor: "android".to_string() , date: 6.7 }
    }

    pub fn use_linux(&self) -> System {
        System::Linux
    }
}

fn mind() {
    let a = System::Linux;
    let system = a.use_linux();
    println!("{:?}", system);


    let android = System::use_android(&System::Linux);
    println!("{:?}", android);

    let ios = System::IOS("hello".to_string(), 6.7);
    println!("{:?}", ios);
}
