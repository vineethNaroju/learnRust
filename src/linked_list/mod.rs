
pub mod dsll {

    pub mod boxed {
        pub struct Lnode {
            val: i32,
            next: Option<Box<Lnode>>
        }
    
        impl Drop for Lnode {
            fn drop(&mut self) {
                println!("Dropping value:{}", self.val);
            }
        }
    
    
        impl Lnode {
            pub fn new(val: i32) -> Lnode {
                Lnode { val, next: None }
            }
    
            pub fn add_next(&mut self, node: Lnode) {
                self.next = Some(Box::new(node));
            }
    
            pub fn print(&self) {
                let mut curr = self;
    
                loop {
                    println!("val: {}", curr.val);
    
                    match &curr.next {
                        None => println!("End of Lnodes"),
                        Some(thing ) => {
                            curr = thing;
                        } 
                    }
                }
            }
    
        }
    
        pub fn demo() {
            let mut a = Lnode::new(10);
    
            let mut b = Lnode::new(20);
    
            a.add_next(b);
    
            
            // println!("b val:{}", b.val); 
            // this can't be done since in above statement, we are moving b value's ownership into a
    
            let c = Lnode::new(30);
    
        }
    }

    pub mod rced {
        use std::{rc::{Rc, self}, fmt::Display};

        struct Lnode{
            val: i32,
            next: Option<Rc<Lnode>>
        }

        impl Drop for Lnode {
            fn drop(&mut self) {
                println!("Dropping value:{}", self.val);
            }
        }

        impl Lnode {
            pub fn new(val: i32) -> Self {
                Lnode { val: val, next: None }
            }

            pub fn add_next(mut self, nxt: Rc<Lnode>) -> Self {
                self.next = Some(nxt);
                return self;
            }

            pub fn print(&self) {
                let mut curr = self;
                loop {
                    println!("Lnode:{}", curr.val);

                    match &curr.next {
                        Some(nxt) => {
                            curr = nxt;
                        },
                        None => {
                            break;
                        }
                    }
                }
            }

        }

        pub fn demo() {
            let mut a = Lnode::new(10);

            let b = Rc::new(Lnode::new(20));

            a = a.add_next(Rc::clone(&b));

            a.print();
        }



    }
    
    

}