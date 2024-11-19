pub struct Bignum {
    digits: Vec<u8>
}

impl Bignum {
    pub fn new(number: &str) -> Self {
        Bignum {
            digits: number.chars()            // 遍历字符串中的字符
                .filter_map(|c| c.to_digit(10))   // 转换每个字符为数字（如果是数字）
                .map(|d| d as u8)                 // 将数字转换为 u8 类型
                .collect::<Vec<u8>>()           // 收集到一个 Vec<u8>,
        }
    }
    pub fn print(&self) {
        print!("{}", self.digits.iter()
                        .map(|n| n.to_string()) // 将每个数字转换为字符串
                        .collect::<Vec<String>>() // 收集成一个 Vec<String>
                        .join("") // 拼接为一个字符串
              );
    }
}