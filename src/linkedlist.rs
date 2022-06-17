use std::{
    borrow::BorrowMut,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    Helmet(bool),              //is damaged?
    LeftThrusters(bool, i32),  //is damaged? How much power left?
    RightThrusters(bool, i32), //is damaged? How much power left?
    LeftRepulsor(bool, i32),   //is damaged? How much power left?
    RightRepulsor(bool, i32),  //is damaged? How much power left?
    ChestPiece(bool, i32),     //is damaged? How much power left?
    Missiles(i32),             //how many missiles left?
    ArcReactor(i32),           // How much power left?
    Wifi(bool),                // connected to wifi?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Armor {
    pub component: Component,
    pub version: i32,
}

// Part 2

// Students should fill in the Link type themselves. The Node and List types are given as is.
type Link = Option<Arc<RwLock<Node>>>;

struct Node {
    data: Armor,
    rest: Link,
}

#[derive(Clone)]
pub struct List {
    head_link: Link,
    size: usize,
} 

impl List {
    pub fn new() -> Self {
    	List{head_link: None, size: 0}
	}

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn peek(&self) -> Option<Armor> {
	    match &self.head_link {
                None => return None,
                Some(x) => Some(x.read().unwrap().data), //reads Armor from head_link
        }
    }

    pub fn push(&mut self, component: Armor) {
            let head:Link = Some(Arc::new(RwLock::new(Node{data: component, rest: self.head_link.take()}))); //take might remove contents of head too early (changed to None)
            self.head_link = head;
            self.size += 1;
    }
    


    pub fn pop(&mut self) -> Option<Armor> {
	    //let mut grab:Option<Armor> = None;
	    if let Some(x) = &self.head_link {
	        let new_head = x.read().unwrap().rest.clone();
            let grab = Some(x.read().unwrap().data); 
            self.head_link = new_head;
            self.size -= 1;
	        return grab;
	    }else{
		    return None;
	    }
    }
}

// Part 3
/*impl<'a,List> Iterator for Iter<'a,List>{
    type Item = &List.as_ref().unwrap().read().unwrap();
    
    fn next(&mut self) -> Option<Self::Item>{
        self.rest.map(|node| {
            self.rest = node.rest;//.as_ref().unwrap().read().unwrap();
            &node.data
        })

    }
}*/

#[derive(Clone)]
pub struct Suit {
    pub armor: List,
    pub version: i32,
}

impl Suit {
    pub fn is_compatible(&self) -> bool {
	let mut upgraded = true;
	let vers = self.version;
    let clon = self.armor.head_link.as_ref().clone();
	let lock = &clon.unwrap().read().unwrap();
    //let mut clon2 = &
    //let link:&Link = self.armor.head_link.as_ref();//.unwrap().read().unwrap().data;
   // let mut res:&Link = &lock.rest;
    
    //let mut focus = self.armor.head_link;
    loop{
        
        if vers != lock.data.version{        
            upgraded = false;
        }

        if let None = lock.rest{
            return upgraded;
        }
        clon = lock.rest.as_ref(); //.clone(); //.as_ref().unwrap().read().unwrap();  
    }
    return upgraded;
    }

    pub fn repair(&mut self) {
	
    //let mut arm:Armor = lock.data;
    //let mut res:Link = lock.rest;
 //struct RwLockWriteGuard 
    {let mut lock = self.armor.head_link.as_ref().unwrap().write().unwrap();
    let mut comp = &lock.data.component;
    for i in 1..self.armor.size{
		{match lock.data.component{
		
		 Component::Helmet(x) => if x == true{ lock.data.component = Component::Helmet(false)},           
		 Component::LeftThrusters(b, n) => if b == true{ lock.data.component = Component::LeftThrusters(false, 100)},
		 Component::RightThrusters(b, n) => if b == true{ lock.data.component = Component::RightThrusters(false,100)},
		 Component::LeftRepulsor(b, n) => if b == true{ lock.data.component = Component::LeftRepulsor(false, 100)},
		 Component::RightRepulsor(b, n) => if b == true{ lock.data.component = Component::RightRepulsor(false,100)}, 
   		 Component::ChestPiece(b, n) => if b == true{ lock.data.component = Component::ChestPiece(false, 100)},
    	 Component::Missiles(n) => if i ==0{ lock.data.component = Component::Missiles(10)},  
    	 Component::ArcReactor(n) => if n < 15{ lock.data.component = Component::ArcReactor(100)},
		 Component::Wifi(b) => if b == false{ lock.data.component = Component::Wifi(true)}, 

		}} 
		lock = lock.rest.as_ref().unwrap().write().unwrap();
	}
    }
    //drop(lock);
    }
}
