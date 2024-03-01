//! Rust代码风格
//! 参考：https://chuxiuhong.com/chuxiuhong-rust-patterns-zh/intro.html
fn main() {
    println!("Hello, world!");
    let curious = "curious";
    println!("{}", three_vowels(curious));

    let sentence_string = "Once upon a time, there was a friendly curious crab named Ferris";
    for word in sentence_string.split(' ') {
        println!("{} has three consecutive vowels!", word);
    }
}

/// 1. 参数最好使用借用类型
/// &str > &String
/// &[T] > &Vec<T>
/// &T > &Box<T>

fn three_vowels(words: &str) -> bool {
    let mut vomel_count = 0;
    for index in words.chars() {
        match  index {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vomel_count += 1;
                if vomel_count >= 3 {
                    return true;
                }
            }
            _ => vomel_count = 0
        }
    }
    false
}
