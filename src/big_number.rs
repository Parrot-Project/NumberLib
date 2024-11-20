pub struct Bignum {
    digits: Vec<u8>,
    trailing_zeros: usize,
}

impl Bignum {
    pub fn new<T: ToString>(number: T) -> Self {
        // 转换为string类型
        let number_str = number.to_string();
        
        // 去除前导零
        let trimmed_str = number_str.trim_start_matches('0');
        
        // 计算后导零的数量
        let mut trailing_zeros = number_str.len() - trimmed_str.len();
        
        // 将去除前导零后的字符串转换为 Vec<u8>
        let mut digits: Vec<u8> = trimmed_str.chars()      // 把字符串转换为字符
            .filter_map(|c| c.to_digit(10))// 把字符转换为数字
            .map(|n| n as u8)                     // 把u32转换为u8
            .collect(); // 存储到vec中
        
        // 处理特殊情况：如果数字全是零
        if digits.is_empty() {
            digits = vec![0];  // 数字全是零时，表示为 [0]
            trailing_zeros = 0;
        }
        
        Bignum {
            digits,
            trailing_zeros,
        }
    }

    // 获取数字的有效位数
    pub fn valid_digits(&self) -> &Vec<u8> {
        &self.digits
    }

    // 获取后导零的数量
    pub fn trailing_zeros(&self) -> usize {
        self.trailing_zeros
    }
    
    // 输出
    pub fn print(&self) {
        // 获取实际位的字符串
        let mut result: String = self.digits.iter()
            .map(|&digit| char::from_digit(digit as u32, 10).unwrap())
            .collect();

        // 添加后导零
        result.push_str(&"0".repeat(self.trailing_zeros));

        // 输出
        println!("{}\n", result);
    }
}