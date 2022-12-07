use core::panic;
use std::collections::HashMap;

use itertools::Itertools;
use reformation::Reformation;

#[derive(Debug, Clone)]
struct Node<'a> {
    name: String,
    contents: Element<'a>,
    parent: &'a Option<Box<Node<'a>>>,
}

#[derive(Debug, Clone)]
enum Element<'a> {
    Directory(Vec<Box<Node<'a>>>),
    File(u64),
}

#[derive(Debug, Reformation, Clone)]
#[reformation()]
enum Command {
    #[reformation(r"\$ cd {}")]
    Cd(String),
    #[reformation(r"\$ ls")]
    Ls,
    #[reformation(r"dir {}")]
    Dir(String),
    #[reformation(r"{} {}")]
    File(u64, String),
}

#[derive(Debug, Clone)]
struct State<'a> {
    current_node: Option<Box<Node<'a>>>,
    root_node: Option<Box<Node<'a>>>,
}

fn main() -> color_eyre::Result<()> {
    let commands = include_str!("../../input/day7.txt")
        .lines()
        .map(|line| Command::parse(line).unwrap())
        .collect_vec();

    let mut index = 0;
    let mut state = State {
        current_node: None,
        root_node: None,
    };
    while index < commands.len() {
        match &commands[index] {
            Command::Cd(path) => {
                match path.as_str() {
                    "/" => {
                        if state.root_node.is_none() {
                            state.root_node = Some(Box::new(Node {
                                name: "/".to_owned(),
                                contents: Element::Directory(vec![]),
                                parent: &None,
                            }));
                        }
                        state.current_node = state.root_node;
                    }
                    ".." => {
                        state.current_node = *state.current_node.unwrap().parent;
                    }
                    _ => {
                        state.current_node = match state.current_node.unwrap().contents {
                            Element::Directory(children) => children
                                .iter()
                                .find(|child| &child.name == path)
                                .map(|&x| x),
                            Element::File(_) => panic!("Current node should never be a file"),
                        };
                    }
                };
                index += 1;
            }
            Command::Ls => {
                let (new_index, nodes) =
                    read_directory(&commands, index, &state.current_node);
                index = new_index;
                state.current_node.unwrap().contents = Element::Directory(nodes);
            }
            _ => panic!("Unexpected command"),
        };
    }

    // dbg!(&state.tree);
    let first_node = state.root_node.unwrap();
    let sizes = find_node_size(&first_node);

    let result: u64 = sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum();
    dbg!(result);

    Ok(())
}

fn read_directory<'a>(
    commands: &[Command],
    index: usize,
    parent: &'a Option<Box<Node<'a>>>,
) -> (usize, Vec<Box<Node<'a>>>) {
    let mut current_index = index + 1;
    let mut nodes = vec![];
    loop {
        match commands.get(current_index) {
            Some(Command::Dir(name)) => nodes.push(Box::new(Node {
                name: (*name.clone()).to_string(),
                contents: Element::Directory(vec![]),
                parent: parent,
            })),
            Some(Command::File(size, name)) => nodes.push(Box::new(Node {
                name: (*name.clone()).to_string(),
                contents: Element::File(*size),
                parent: parent,
            })),
            _ => return (current_index, nodes),
        }
        current_index += 1
    }
}

fn find_node_size(node: &Node) -> HashMap<String, u64> {
    match &node.contents {
        Element::Directory(children) => {
            let mut sizes: HashMap<String, u64> =
                children.into_iter().flat_map(|child| find_node_size(child)).collect();
            let dir_size = children
                .iter()
                .map(|child| sizes.get(&child.name).unwrap())
                .sum();
            sizes.insert(node.name.clone(), dir_size);
            sizes
        }
        Element::File(size) => HashMap::from([(node.name.clone(), *size)]),
    }
}
