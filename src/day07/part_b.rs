use itertools::fold;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum NodeType {
    File,
    Directory,
}

#[derive(Debug)]
struct Node {
    name: String,
    ntype: NodeType,
    size: i32,
}

impl Node {
    fn new(name: String, ntype: NodeType, size: i32) -> Node {
        Node { name, ntype, size }
    }
}

pub fn part_b(contents: &str) -> i32 {
    lazy_static! {
        static ref RE_CMD: Regex = Regex::new(r"^\$ ([^\s]+)(?:\s([^\s]+))?$").unwrap();
        static ref RE_DIR: Regex = Regex::new(r"^dir (.*)").unwrap();
        static ref RE_FILE: Regex = Regex::new(r"^(\d{1,}) (.*)").unwrap();
    }

    let mut operations: Vec<Node> = Vec::new();
    let file_system_size = 70000000;
    // let space_used_1 = 48381165;
    let space_used = 41111105;
    // let mut space_to_delete_1 = space_used_1;
    let mut space_to_delete = space_used;
    let file_space_needed = 30000000 - (file_system_size - space_used);

    let mut total_sum = 0;

    for stdin in contents.lines() {
        if RE_CMD.is_match(stdin) {
            let params = RE_CMD.captures(stdin).unwrap();
            match params.get(1).unwrap().as_str() {
                "cd" => {
                    match params.get(2).unwrap().as_str() {
                        ".." => {
                            if operations.len() > 0 {
                                let folder_size = operations.pop().unwrap().size;
                                let operations_len = operations.len();
                                operations[operations_len - 1].size += folder_size;
                                if folder_size <= 100000 {
                                    total_sum += folder_size;
                                }
                                if folder_size < space_to_delete && folder_size >= file_space_needed
                                {
                                    space_to_delete = folder_size;
                                }
                            }
                        }
                        _ => {
                            let folder = Node::new(
                                String::from(params.get(2).unwrap().as_str()),
                                NodeType::Directory,
                                0,
                            );
                            operations.push(folder);
                        }
                    };
                }
                _ => (),
            };
        } else if RE_FILE.is_match(stdin) {
            let file_size = RE_FILE
                .captures(stdin)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let operations_len = operations.len();
            operations[operations_len - 1].size += file_size;
        }
    }

    while operations.len() > 0 {
        let folder_size = operations.pop().unwrap().size;
        let operations_len = operations.len();
        if operations.len() > 0 {
            operations[operations_len - 1].size += folder_size;
        }
        if folder_size <= 100000 {
            total_sum += folder_size;
        }
        if folder_size < space_to_delete && folder_size >= file_space_needed {
            space_to_delete = folder_size;
        }
        // println!("{:?}", operations)
    }

    return space_to_delete;
}
