use rand::random;

pub fn impl_three() {
    println!("-----------impl_three------------");
    let mut f = File;
    let mut buffer = vec![];

    read(&mut f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }

}

static mut ERROR: isize = 0;

struct File;


fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        }
    }
    0
}

