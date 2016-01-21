use std::cmp::Ordering;


type Child<T> = Option<Color<Box<Node<T>>>>;

#[derive(Debug, Clone)]
enum Color<T> {
	Black(T),
	Red(T)
}

#[derive(Debug, Clone)]
struct Tree<T> {
    parent: Child<T>
}

#[derive(Debug, Clone)]
struct Childs<T>(Child<T>, Child<T>);

#[derive(Debug, Clone)]
struct Node<T> {
	val: T,
	childs: Option<Childs<T>>
}

impl <T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        true
    }
}
impl <T: PartialEq + PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl <T> Tree<T> {
    fn new(v: T) -> Tree<T> {
        Tree{ parent: Some(Color::Black(Box::new(Node::new(v,None)))) }
    }
    
    fn insert(self, v: T) {
        match self.parent {
            Some(c) => {
                match c {
                    Color::Black(x) => println!("Black"),
                    Color::Red(x) => println!("Red")
                }
            }
            None => println!("None")             
        }
    }
}


impl<T> Node<T> {
	fn new (val: T, childs: Option<Childs<T>>) -> Node<T> {
		Node {
			val: val,
			childs: childs
		}
	}
}


fn main() {

    
}



#[test]
fn sample_insert() {
    let s = &["aaa", "bbb", "ccc"];
	let mut parent  = Tree::new(s.first().unwrap().to_string());
    for v in s {
        parent.clone().insert(v.to_string())
    }
}
