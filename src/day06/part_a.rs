use std::collections::HashSet;

pub fn part_a(contents: &str) -> i32 {
    let mut buffer_start = 0;
    let mut buffer_end = 4;
    while buffer_start < contents.len() {
        let buffer = &contents[buffer_start..buffer_end]
            .chars()
            .into_iter()
            .collect::<HashSet<_>>();
        if buffer.len() == 4 {
            break;
        }
        buffer_start += 1;
        if buffer_end < contents.len() {
            buffer_end += 1
        }
    }
    return buffer_end as i32;
}
