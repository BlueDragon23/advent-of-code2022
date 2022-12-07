use std::collections::HashMap;

use itertools::Itertools;
use reformation::Reformation;

#[derive(Debug, Clone)]
struct Node<'a> {
    name: String,
    contents: Element<'a>
}

#[derive(Debug, Clone)]
enum Element<'a> {
    Directory(Vec<&'a Node<'a>>),
    File(u64)
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
    File(u64, String)
}

#[derive(Debug, Clone)]
struct State<'a> {
    current_node: Option<Node<'a>>,
    parent_node: Option<Node<'a>>,
    root_node: Option<Node<'a>>
}

fn main() -> color_eyre::Result<()> {
    let mut state = State { current_node: None, parent_node: None, root_node: None };
    include_str!("../../input/day7.txt")
        .lines()
        .batching(|it| {
            match Command::parse(it.next().unwrap()).unwrap() {
                Command::Cd(name) => {
                    if name == ".." {
                        Some(state.parent_node)
                    } else if name == "/" {
                        if let None = state.root_node {
                            state.root_node = Some(Node {name: "/".to_string(), contents: Element::Directory(vec![])});
                        };
                        Some(state.root_node)
                    } else {
                        state.parent_node = state.current_node;
                        Some(Node {name, contents: Element::Directory(vec![])})
                    }
                },
                Command::Ls => {
                    // no action
                    Some()
                },
                Command::Dir(name) => {
                    state.tree.insert(name.clone(), Node {name, contents: Element::Directory(vec![], 0)});
                },
                Command::File(size, name) => {
                    let child = Node {name: name.clone(), contents: Element::File(size)};
                    state.tree.insert(name.clone(), child.clone());
                    let n = state.tree.get_mut(&state.current_node.name).unwrap();
                    if let Element::Directory(mut children, _) = n.contents {
                        children.push(child);
                    }
                },
            }
        });

    // dbg!(&state.tree);
    let first_node = state.tree.get("/").unwrap().clone();
    let sizes = find_node_size(&first_node);

    let result: u64 = sizes.iter().filter(|(_, &size)| size < 100000).map(|(_, size)| size).sum();
    dbg!(result);

    Ok(())
}

fn find_node_size(node: &Node) -> HashMap<String, u64> {
    match &node.contents {
        Element::Directory(children, _) => {
            let mut sizes: HashMap<String, u64> = children
                .iter()
                .flat_map(find_node_size)
                .collect();
            let dir_size = children.iter().map(|child| sizes.get(&child.name).unwrap()).sum();
            sizes.insert(node.name.clone(), dir_size);
            sizes
        },
        Element::File(size) => {
            HashMap::from([(node.name.clone(), *size)])
        },
    }
}