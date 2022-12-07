use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
enum Type {
    Dir,
    File,
}

type Content = HashMap<String, Node>;

#[derive(Debug)]
struct Node {
    t: Type,
    size: usize,
    content: RefCell<Content>,
}

impl Node {
    fn new(t: Type, size: usize) -> Node {
        Node {
            t,
            size,
            content: RefCell::new(HashMap::new()),
        }
    }
}

fn build_tree(commands: &[String], start: usize) -> (Content, usize, usize) {
    let mut content: Content = HashMap::new();
    let mut i = start;
    let mut dir_size = 0;
    while i < commands.len() {
        let mut cmd = commands[i].split(" ");
        match cmd.next().unwrap() {
            "$" => match cmd.next().unwrap() {
                "cd" => match cmd.next().unwrap() {
                    ".." => return (content, i - start + 1, dir_size),
                    s => {
                        let res = build_tree(commands, i + 1);
                        content.get(s).unwrap().content.borrow_mut().extend(res.0);
                        i += res.1;
                        content.get_mut(s).unwrap().size = res.2;
                        dir_size += res.2;
                    }
                },
                "ls" => {}
                _ => {}
            },
            "dir" => {
                let name = cmd.next().unwrap();
                content.insert(String::from(name), Node::new(Type::Dir, 0));
            }
            s => {
                let name = cmd.next().unwrap();
                let size = s.parse().unwrap();
                dir_size += size;
                content.insert(String::from(name), Node::new(Type::File, size));
            }
        };
        i += 1;
    }
    (content, commands.len() - start, dir_size)
}

fn filter_dir(node: &Node, size: usize) -> usize {
    match node.t {
        Type::File => {
            if node.size <= size {
                0
            } else {
                0
            }
        }
        Type::Dir => {
            let mut content_s = if node.size <= size { node.size } else { 0 };
            for c in node.content.borrow().values() {
                content_s += filter_dir(c, size);
            }
            content_s
        }
    }
}

fn part_one(input: &str) -> Option<usize> {
    let mut root = Node::new(Type::Dir, 0);
    {
        let (tree, _, root_size) = build_tree(aoc::utils::read_list(input).as_slice(), 1);
        root.size = root_size;
        root.content.get_mut().extend(tree);
    }
    Some(filter_dir(&root, 100000))
}

fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2022, 07);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2022, 07);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2022, 07);
        assert_eq!(part_two(&input), None);
    }
}
