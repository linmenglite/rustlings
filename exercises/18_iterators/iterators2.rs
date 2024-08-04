// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {first.to_uppercase().to_string()+chars.as_str()},
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut ret = Vec::new(); // 调用 Vec::new() 来初始化空向量  
  
    for word in words {  
        let mut chars = word.chars(); // 使用不同的变量名来避免冲突  
  
        if let Some(first) = chars.next() { // 安全地获取第一个字符  
            let first_upper = first.to_uppercase().to_string();  
            let rest_of_word = chars.as_str(); // 获取剩下的字符  
            ret.push(first_upper + rest_of_word); // 拼接并添加到结果向量中  
        } else {  
            // 如果单词为空，您可能想要添加空字符串或处理这种情况  
            ret.push(String::new());  
        }  
    }  
  
    ret
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    let mut ret =String::new();
    for words in words {
        let mut c = words.chars();
        match c.next() {
            None => {},
            Some(first) => {ret+=&(first.to_uppercase().to_string()+c.as_str());},
        }
    }
    ret.to_string()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
