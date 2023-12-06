pub fn higher() {
    println!();
    handle_csv();
}

fn handle_csv() {
    let penguin_data = "\n
    common name,length (cm)\n
    Little penguin,33\n
    Yellow-eyed penguin,65\n
    Fiordland penguin,60\n
    Invalid,data\n
    ";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        //println!("{} {:?}", i,record);
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}