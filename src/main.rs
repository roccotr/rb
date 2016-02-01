use std::cmp::Ordering;

type Child<T> = Option<Box<Node<T>>>;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
enum Color {
	Black,
	Red
}

#[derive(Debug, Clone)]
struct Tree<T> {
    par: Child<T>
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
struct Childs<T>(Child<T>, Child<T>);

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
struct Node<T> {
	val: T,
    color: Color,
    par: Child<T>,
	childs: Option<Childs<T>>
}

/*
impl <T> PartialEq for Box<Node<T>> {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}
impl <T: PartialEq + PartialOrd> PartialOrd for Box<Node<T>> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}*/

/*
impl <T: Eq + Ord> Ord for Box<Node<T>> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl <T: Eq + Ord> Eq for Box<Node<T>> {}
*/
/*
impl <T> Iterator for Box<Node<T>> {
    type Item = Box<Node<T>>;
    fn next(&mut self) -> Child<T> {
        unimplemented!()
    }
}
*/
impl <T: Eq + Ord + Clone> Tree<T> {
    fn new(v: T) -> Tree<T> {
        Tree{ par: Some(Box::new(Node::new(v, Color::Black, None, None))) }
    }
    
    fn insert(&mut self, v: T) {
        let p = self.par.clone();
        let node = Tree::search(p, v);
        /*match self.parent {
            Some(c) => println!("Some"),
            None => println!("None")             
        }*/
    }
    
    fn search(node: Child<T>, v: T) -> Option<Child<T>> {
        match node {
            Some(n) => {
                    match n.val.cmp(&v) {
                        Ordering::Less => Some(n),
                        Ordering::Greater => Some(n),
                        Ordering::Equal => Some(n)
                    }
                },
            None => None
        }
    }
}


impl<T: Eq + Ord> Node<T> {
	fn new (val: T,  color: Color, par: Child<T>, childs: Option<Childs<T>>) -> Node<T> {
		Node {
			val: val,
            color: color,
            par: par,
			childs: childs
		}
	}
}


fn main() {

    
}



#[test]
fn sample_insert() {
    let s = &["aaa", "bbb", "ccc"];
	let mut parent: Tree<String>  = Tree::new(s.first().unwrap().to_string());
    for v in s {
        let mut p = parent.clone();
        p.insert(v.to_string())
    }
}
