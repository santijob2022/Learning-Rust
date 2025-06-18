
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
        
    

        #[derive(Debug)]
        struct  House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>
        }

        #[derive(Debug)]
        struct  Furniture {
            id: String,
            description: String,
            house: Weak<House>
        }

        let house_1: Rc<House> = Rc::new(House{
            address_number: 123,
            street: "elm_street".to_string(),
            furniture: RefCell::new(vec!())
        });

        let table: Rc<Furniture> = Rc::new(Furniture{
            id: "table1".to_string(),
            description: "leaving room table".to_string(),
            house: Rc::downgrade(&house_1)
        });

        let chair = Rc::new(Furniture{
            id: "chair1".to_string(),
            description: "office chair".to_string(),
            house: Rc::downgrade(&house_1)
        });

        house_1.furniture.borrow_mut().push(Rc::clone(&table));
        house_1.furniture.borrow_mut().push(Rc::clone(&chair));

        dbg!(house_1);


    }
}