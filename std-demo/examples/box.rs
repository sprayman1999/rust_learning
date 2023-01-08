use std::fmt;

#[derive(Debug)]
struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct LinkedList<T>{
    head: Option<Box<Node<T>>>
}

impl<T> Node<T>{
    fn new(data: T) -> Self{
        Self { data: data,next: None }
    }
}
impl<T> LinkedList<T>{
    fn new() -> Self{
        Self { head: None }
    }

    fn push(&mut self,data: T) -> &mut Self{
        let mut new_node = Node::new(data);
        match self.head{
            Some(_) => {
                new_node.next = self.head.take();
                self.head = Some(Box::new(new_node));
            }
            None => {
                self.head = Some(Box::new(new_node));
            }
        }
        self
    }

    fn pop(&mut self) -> T{
        match self.head.take(){
            Some(node) => {
                let data: T = node.data;
                self.head = node.next;
                data
            }
            None => { panic!("LinkedList's head is empty") }
        }
    }

    fn length(&mut self) -> i32{
        match &self.head{
            Some(node) => {
                let mut count = 1;
                let mut tmp_node = node;
                while tmp_node.next.is_some(){
                    tmp_node = tmp_node.next.as_ref().unwrap();
                    count += 1;
                };
                count
            }
            None => { 0 }
        }
    }
}

fn main(){
    let mut linked_list: LinkedList<i32> = LinkedList::new();
    linked_list.push(1).push(2).push(3);
    println!("length: {}\tcontent: {:?}",linked_list.length(),linked_list);
    linked_list.pop();
    linked_list.pop();
    println!("length: {}\tcontent: {:?}",linked_list.length(),linked_list);
}