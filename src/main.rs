#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

use std::rc::Rc;
use std::cmp::Ordering;
use log::*;
use std::fmt::{Display, Debug};

const LOG_LEVEL: log::LogLevelFilter = log::LogLevelFilter::Debug;
const LOG_FILE_NAME: &'static str = "./logs/log.log";

type Child<'a, T> = Rc<Node<'a, T>>;
type OptionChild<'a, T> = Option<Child<'a, T>>;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
enum Color {
	Black,
	Red
}

#[derive(Debug, Clone)]
struct Tree<'a, T: 'a> {
    par: Child<'a, T>
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
struct Childs<'a, T: 'a>(Option<&'a Child<'a, T>>, Option<&'a Child<'a, T>>);

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
struct Node<'a, T: 'a> {
	val: Option<T>,
    color: Color,
    par: Option<&'a Child<'a, T>>,    
	childs: Childs<'a, T>
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
impl <'a, T: 'a + Eq + Ord + Debug + Display> Childs<'a, T> {
    fn new(par: Option<&'a Child<'a, T>>, color: Color) -> Childs<'a, T> {
        Childs(
            None,
            None
        )
    }
    
    fn left(&mut self) -> Option<&'a Child<'a, T>> {
        self.0
    }
    
    fn right(&mut self) -> Option<&'a Child<'a, T>> {
        self.1
    }
} 


impl <'a, T: 'a + Eq + Ord + Clone + Display + Debug> Tree<'a, T> {
    fn new(v: T) -> Tree<'a, T> {
        Tree{ par: Node::new(None, Color::Black, None) }
    }
    
    fn insert(&mut self, v: T) {
        match Rc::get_mut(&mut self.par) {
            Some(n) => {n.insert(v); Ok(())},
            None =>  Err("err")
        };
    }
    
    fn search(node: &Child<T>, val: T) -> Result<(Ordering,Child<'a, T>),String> {
        unimplemented!()
    }
}


impl<'a, T: 'a + Eq + Ord + Debug + Display> Node<'a, T> {
	fn new (val: Option<T>,  color: Color, par: Option<&'a Child<'a, T>>) -> Rc<Node<'a, T>> {
		Rc::new(Node {
			val: val,
            color: color,
            par: par,
			childs: Childs::new(par, Color::Red)
		})
	}
    
    fn insert(&mut self, val: T) {
        match self.val {
            Some(ref v) => { 
                match v.cmp(&val) {
                    Ordering::Less => {
                        debug!("Less {} - {}",v,val);
                        match self.childs.left() {
                            Some(c) => match Rc::get_mut(&mut c) {
                                Some(c) => c.insert(val),
                                None => error!("err") 
                            },
                            None => error!("err")
                        }
                    },
                    Ordering::Greater => {
                        debug!("Greater {} - {}",v,val);
                        match self.childs.right() {
                            Some(mut c) => match Rc::get_mut(&mut c) {
                                Some(c) => c.insert(val),
                                None => error!("err") 
                            },
                            None => error!("err")
                        }
                    },
                    Ordering::Equal => {
                        debug!("Equal {} - {}",v,val);
                    }
                }                
            },
            None => {
                self.val = Some(val);
                self.childs = Childs::new(None, Color::Red);
            }
        };
    }
}


fn init_logger() -> std::io::Result<()> {

    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            format!("[{}] - {} - {}", time::now().strftime("%Y-%m-%d:%H:%M:%S").unwrap(), level, msg)
         }),
        output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file(LOG_FILE_NAME)],
        level: LOG_LEVEL,
    };
    if let Err(e) = fern::init_global_logger(logger_config, LOG_LEVEL) {
        panic!("Failed to initialize global logger: {}", e);
    }
    Ok(())
}

fn main() {
    init_logger();
    let s = &["aaa", "bbb", "ccc"];
	let mut parent: Tree<String>  = Tree::new(String::new());
    for v in s {
        let mut p = parent.clone();
        p.insert(v.to_string())
    }
    
}



#[test]
fn sample_insert() {
    let s = &["aaa", "bbb", "ccc"];
	let mut parent: Tree<String>  = Tree::new(String::new());
    for v in s {
        let mut p = parent.clone();
        p.insert(v.to_string())
    }
}
