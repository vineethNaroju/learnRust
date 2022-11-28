pub mod store {
    use std::rc::Rc;

    pub struct Dbstore<K, V> {
        keys: Vec<K>,
        values: Vec<Rc<V>>
    }
   
    impl<K: std::cmp::Eq, V> Dbstore<K,V> {

        pub fn new() -> Dbstore<K, V> {
            Dbstore { keys: vec![], values: vec![] }
        }

        pub fn get(&self, key: K)  -> Option<Rc<V>> {

            let n = self.keys.len();
            let mut i = n-1;

            loop {

                if self.keys[i] == key {
                    return Some(Rc::clone(&self.values[i]));
                }

                if i == 0 {
                    break;
                }

                i -= 1;
            }

            return None;
        }

        pub fn set(&mut self, key: K, val: Rc<V>) {
            self.keys.push(key);
            self.values.push(Rc::clone(&val));
        }
    }


    pub fn demo() {
        let mut db = Dbstore::new();


        let ak = Rc::new("akey".to_string());
        let av = Rc::new("aval".to_string());

        db.set(Rc::clone(&ak), Rc::clone(&av));

        println!("{}", Rc::clone(&ak) == Rc::clone(&ak));

        println!("ak:{},av:{}", ak, av);

        match db.get(Rc::clone(&ak)) {
            Some(val) => {
                println!("db.get({}):{}", av, val);
            },
            None => {
                println!("db.get({}): Not found", av);
            }
        }
    }
 
}


