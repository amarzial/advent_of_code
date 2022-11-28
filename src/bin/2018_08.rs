#[derive(Debug)]
struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<i32>,
    size: usize,
}

type Serial = [i32];

impl Node {
    pub fn build(input: &Serial) -> Node {
        let children_n = input[0];
        let metadata_n = input[1];
        let mut position = 2;

        let mut n = Node {
            children: vec![],
            metadata: vec![],
            size: 0,
        };
        for _c in 0..children_n {
            let data = &input[position..];
            let child = Node::build(data);
            position += child.size;
            n.children.push(Box::new(child));
        }

        for i in 0..metadata_n {
            n.metadata.push(input[position + i as usize]);
        }
        n.size = position + metadata_n as usize;
        n
    }

    pub fn count(&self) -> i32 {
        let mut total = self.metadata.iter().sum();
        for c in self.children.iter() {
            total += c.count();
        }
        total
    }

    pub fn value(&self) -> i32 {
        let mut total = self.metadata.iter().sum();

        if self.children.len() > 0 {
            total = 0;
            for m in self.metadata.iter() {
                match self.children.get(*m as usize - 1) {
                    Some(c) => {
                        total += c.value();
                    }
                    None => {}
                }
            }
        }
        total
    }
}

fn part_one(input: &str) -> Option<String> {
    let tokens = aoc::utils::read_line::<i32>(input, " ");
    let root = Node::build(tokens.as_slice());

    Some(root.count().to_string())
}

fn part_two(input: &str) -> Option<String> {
    let tokens = aoc::utils::read_line::<i32>(input, " ");
    let root = Node::build(tokens.as_slice());

    Some(root.value().to_string())
}

fn main() {
    let input = aoc::utils::load_input("inputs", 2018, 08);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::utils::load_input("examples", 2018, 08);
        assert_eq!(part_one(&input), Some(138.to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::utils::load_input("examples", 2018, 08);
        assert_eq!(part_two(&input), Some(66.to_string()));
    }
}
