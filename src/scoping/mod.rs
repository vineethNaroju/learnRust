
pub mod notes {
    use std::i32;

    pub fn raii() {
        fn create_box() {
            let _box1 = Box::new(10i32);
            // _box1 is destroyed
        }

        let _box2 = Box::new(25i32);

        {
            let _box3 = Box::new(-20i32);
            // _box3 is destroyed
        }

        for _ in 0u32..1_000 {
            create_box();
        }

        // _box2 is destroyed

        struct ToDrop;

        impl Drop for ToDrop {
            fn drop(&mut self) {
                println!("ToDrop is being dropped");
            }
        }

        let x = ToDrop;
        println!("Made a ToDrop");
    }

    pub fn ownership_move() {

        // doing assignments or passing by value, the ownership of resources
        // is transferred. In rust-speak, this is known as *move*


        fn destroy_box(bx : Box<i32>) {
            println!("Destroying a box that contains {}", bx);
        }

        let x = 5u32; // stack allocated integer
        let y = x ; // *Copy*, no resources are moved

        println!("x:{}, y:{}", x, y);

        let a = Box::new(6i32); // a is a pointer to heap allocated integer.

        let b = a; // *Move* a into b
        // pointer address of a is copied into b
        // both are now pointers to same heap allocated data
        // but b owns it.

        // println!("a:{}", a);

        destroy_box(b); // ownership is moved into bx of destroy_box

        // println!("b:{}", b);
    }


    pub fn mutability() {
        // mutability of data can be changed when ownership is transfered

        let immutable_box = Box::new(10i32);

        // *immutable_box = 25;

        let mut mutable_box = immutable_box;

        *mutable_box = 1000;

        println!("mutable box holds value:{}", mutable_box);
    }

    pub fn partial_moves() {
        // within destructuring of single varible, both by-move and by-reference
        // pattern bindings can be used at the same time.
        // In such case, the parent variable can't be used afterwards as a whole,
        // however parts that are referenced can still be used.

        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>
        }

        let person = Person {
            name: String::from("vroom"),
            age: Box::new(20)
        };

        let Person {name, ref age} = person;
        // name is moved out of person but age is referenced

        // println!("{}", person); // can't be used
        // println!("{}", person.name); // can't be used sinc it's moved out of person's ownership into name
        println!("{}", person.age);


    }

    pub fn borrowing() {
        // we'd like to access data without taking ownership over it.
        // rust uses borrowing mechanism, instead of passing objects by value (T)
        // objects can be passed by reference (&T)
        // compiler statically guarantees that references always point to valid objects
        // i.e, while references exist, objects can't be destroyed
        //

        fn eat_box_i32(boxed_i32: Box<i32>) {
            println!("Destroying box that contains {}", boxed_i32);
        }

        fn borrow_i32(borrowed_i32: &i32) {
            println!("This int is: {}", borrowed_i32);
        }

        let boxed_i32 = Box::new(25i32);
        let stacked_i32 = 6_i32;

        borrow_i32(&boxed_i32);
        borrow_i32(&stacked_i32);

        {
            let _ref: &i32 = &boxed_i32;

            // eat_box_i32(boxed_i32); // can't destroy since the inner value is borrowed later in scope.

            borrow_i32(_ref);
        }

        eat_box_i32(boxed_i32);



    }


    pub fn borrowing_mutability() {
        // Mutable data can be mutably borrowed using &mut T => Mutable reference.
        // This gives read / write access to the borrower.
        // In contrastm &T borrows the data via an immutable reference
        // and the borrower can read the data but not modify it.

        #[allow(dead_code)]
        #[derive(Clone, Copy)]
        struct Book {
            author: &'static str,
            title: &'static str,
            year: u32,
        }

        fn borrow_book(book: &Book) {
            println!("We immutably borrowed {} - {} edition", book.title, book.year);
        }

        fn new_edition(book: &mut Book) {
            book.year = 2022;
            println!("We mutably borrowed {} - {} edition", book.title, book.year);
        }

        let immutable_book = Book {
            author: "Douglas",
            title: "Godel, Escher, Bach",
            year: 1979
        };


        let mut mutable_book = immutable_book;

        borrow_book(&immutable_book); // immutably borrow an immutable object

        borrow_book(&mutable_book); // immutably borrow a mutable object;

        new_edition(&mut mutable_book); // mutably borrow a mutable object

        // new_edition(&mut immutable_book); // can't mutably borrow an immutable object



    }

    pub fn borrowing_aliasing() {
        // data while immutably borrowed, the original data can't be mutably borrowed.
        // only one mutable borrow is allowed at a time.
        // the original data can be borrowed again only after the mutable reference has been used
        // for the last time.

        struct Point {
            x: i32,
            y: i32,
            z: i32
        }

        let mut point = Point {x: 0, y:0, z:0};

        let borrowed_point = &point;

        // borrowed_point.x = 30; // doesn't work since it's immutable pointer

        let another_borrow = &point;

        // let mutable_borrow = &mut point; // doesn't work, since the last immutable reference
        // is being used after this

        println!("{}", borrowed_point.x); 
        // println!("{}",mutable_borrow.x);

        let mew = &mut point;

        mew.x = 20;

        println!("{}", mew.x);


    }


    pub fn borrowing_ref_pattern() {
        // In pattern matching / destructing via let, ref can be used to take references to fields
        // of a struct / tuple.
        // ref on left side of assignment is equivalent to & borrow on the right side.

        #[derive(Clone, Copy)]
        struct Point {
            x: i32,
            y: i32
        }


        let c = 'Q';
        let ref ref_c1 = c;
        let ref_c2 = &c;

        let point = Point {x: 20, y: 30};

        let Point {x: ref px, y: py} = point;
        
        println!("{}, {}", px, py);


        

    }

    pub fn lifetimes() {

        fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y .len() {
                x
            } else {
                y
            }
        }


    //     let a = String::from("abcd");
    //     let res;

    //     {
    //         let str2 = String::from("xyz");
    //         res = longest(a.as_str(), str2.as_str());
    //     }


    //    println!("longest:{}", res);
    }

}


