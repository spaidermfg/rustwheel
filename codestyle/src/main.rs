//! Rust代码风格
//! 参考：https://chuxiuhong.com/chuxiuhong-rust-patterns-zh/intro.html
fn main() {
    // 参数最好使用借用类型
    println!("Hello, world!");
    let curious = "curious";
    println!("{} is {}", curious, three_vowels(curious));

    let sentence_string = "Once upon a time, there was a friendly curious crab named Ferris";
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{} has three consecutive vowels!", word);
        }
    }

    // 字符串拼接
    println!("{}",connect_string("world"));
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
                    return true
                }
            }
            _ => vomel_count = 0
        }
    }
    false
}

/// 连接字符串
/// 使用!format更好
/// 使用push效率高
fn connect_string(word: &str) -> String {
    let mut a = format!("Hello {}", word);
    a.push_str(word);
    a.push('!');
    a
}