use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    // 将输入字符串按逗号分割，并收集到HashSet中以去重
    let unique_elements: HashSet<_> = input_str.split(',').collect();
    // 返回HashSet的大小，即不重复元素的个数
    unique_elements.len()
}
