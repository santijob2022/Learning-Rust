
#[cfg(test)]
mod tests {
    use std::rc::{Rc,Weak};
    use std::cell::RefCell;


    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_box_smart_pointer(){        

        #[derive(Debug)]
        struct Node {
            id:u32,
            next: Option<Box<Node>>
        }

        let nodes:Box<Node>  = Box::new(
            Node { id: 0, next: Some(
                Box::new(Node { id: 1, next: Some(Box::new(
                    Node { id: 2, next: None }
                )) })
            ) }
        );

        dbg!(nodes);         

    }

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_reference_counter(){

        // let mut x:Rc<i32> = Rc::new(50);
        // let y:Rc<i32> = Rc::clone(&x);
        // x = Rc::new(70);
        // dbg!(x);
        // dbg!(y);

        let x: Rc<RefCell<i32>> = Rc::new(RefCell::new(50));
        let y: Rc<RefCell<i32>> = Rc::clone(&x);

        *x.borrow_mut() = 70;

        dbg!(x.borrow());
        dbg!(y.borrow());
        dbg!(x);
        dbg!(y);
        
    }

}