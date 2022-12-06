use std::collections::HashSet;

pub fn part_b(contents: &str) -> i32 {
    let mut buffer_start = 0;
    let mut buffer_end = 14;
    while buffer_start < contents.len() {
        let buffer = &contents[buffer_start..buffer_end]
            .chars()
            .into_iter()
            .collect::<HashSet<_>>();
        if buffer.len() == 14 {
            break;
        }
        buffer_start += 1;
        if buffer_end < contents.len() {
            buffer_end += 1
        }
    }
    return buffer_end as i32;
}
