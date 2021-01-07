// Binary tree iimplementation
pub struct Tree{
    pub value: u8,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

impl Tree{
    pub fn insert(&mut self, value: u8){
        if self.value == value {
            return;
        }
        let child = if value < self.value {&mut self.left} else {&mut self.right};
        println!("val{}, valnew{}", self.value, value);
        match child {
            Some(node) => node.insert(value),
            None => {
                let node = Tree{value:value, left:None, right:None};
                *child = Some(Box::new(node));
            }
        }
    }

    pub fn preorder_traversal(&self){
        println!("\t{}", self.value);
        match &self.left {
            Some(node) => node.preorder_traversal(),
            None => ()
        }
        match &self.right {
            Some(node) => node.preorder_traversal(),
            None => ()
        }
    }
    
    pub fn size(&self) -> u8{
        let mut size: u8 = 1;
        match &self.left {
            Some(node) => {size = size + node.size();},
            None => ()
        }
        match &self.right {
            Some(node) => {size = size + node.size();},
            None => ()
        }
        return size;
    }
}

//pub fn preorder_traversal<T: Debug + Display>(tree: Tree<T>){
//    print!("{}", tree.root);
 //   match tree.left {
  //      Some(left) => preorder_traversal(*left),
   //     None => ()
   // }
   // match tree.right {
    //    Some(right) => preorder_traversal(*right),
     //   None => ()
   // }
//}
//struct Node<T>{
//    value: T,
//    left: Node<T>,
//    right: Node<T>,
//}
