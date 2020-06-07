// binary tree algorithms
mod tree;

fn main(){
    let mut t = tree::Tree{value:10, left:None, right:None};
    t.insert(5);
    t.insert(15);
    t.insert(3);
    t.preorder_traversal();
    println!("size: {}", t.size());
}
