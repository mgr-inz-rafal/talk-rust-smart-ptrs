use std::borrow::Cow;
use std::rc::Rc;
use std::rc::Weak;
use std::sync::Arc;

fn main() {
    fn print_int(i: i32) {
        println!("{}", i);
    }

    {
        // ----- Box (unique_ptr) -----
        let my_one = 1;
        print_int(my_one);

        let my_two = Box::new(2);
        print_int(*my_two);
    }

    fn print_int_ref(i: &i32) {
        println!("{}", i);
    }

    {
        // ----- Deref Coercion -----
        let my_one = 1;
        print_int_ref(&my_one);

        let my_two = Box::new(2);
        print_int_ref(&my_two);
    }

    {
        // ----- Drop (~) -----
        struct Beret {};

        impl Drop for Beret {
            fn drop(&mut self) {
                println!("Dropped");
            }
        };

        let _ = Beret {};
        println!("Still alive");
    }

    {
        // ----- Early drop -----
        struct Beret {};

        //impl Copy for Beret {};

        impl Drop for Beret {
            fn drop(&mut self) {
                println!("Dropped");
            }
        };

        let b = Beret {};
        drop(b);
        println!("Alive?");
    }

    {
        // ----- Unsized type - won't compile -----
        /*
        fn foo(seq: [i32]) {
            seq.iter().for_each(|v| println!("{}", v));
        }

        let sequence = [32, 12, 56];
        foo(sequence);
        */
    }

    {
        // ----- Unsized type in-a-box -----
        fn foo(seq: Box<[i32]>) {
            seq.iter().for_each(|v| println!("{}", v));
        }

        let sequence = [32, 12, 56];
        foo(Box::new(sequence));
    }

    {
        // ----- Reference Counted (shared_ptr) -----
        struct Beret {};

        impl Drop for Beret {
            fn drop(&mut self) {
                println!("Dropped");
            }
        };

        let b1 = Rc::new(Beret {});
        let b2 = Rc::clone(&b1);
        let _b3 = b2.clone();
        println!("{}", Rc::strong_count(&b1));
    }

    {
        // ----- Asynchronous Reference Counted (?) -----
        struct Beret {};

        impl Drop for Beret {
            fn drop(&mut self) {
                println!("A Dropped");
            }
        };

        let b1 = Arc::new(Beret {});
        let b2 = Arc::clone(&b1);
        let _b3 = b2.clone();
        println!("{}", Arc::strong_count(&b1));
    }

    {
        // ----- Weak (weak_ptr) -----
        struct Beret {};

        impl Drop for Beret {
            fn drop(&mut self) {
                println!("Dropped");
            }
        };

        let b3: Weak<Beret>;
        {
            let b1 = Rc::new(Beret {});
            let _b2 = Rc::clone(&b1);
            b3 = Rc::downgrade(&b1);
            println!("{}", Rc::strong_count(&b1));
            println!("{}", b3.upgrade().is_some());
        }
        println!("{}", b3.upgrade().is_some());
    }

    {
        // ----- Cow (?) -----
        struct Beret {
            pub i: i32,
        };

        impl Copy for Beret {}

        impl Clone for Beret {
            fn clone(&self) -> Beret {
                println!("Cloning...");
                *self
            }
        }

        let b1 = Beret { i: 1 };
        let mut b2 = Cow::Borrowed(&b1);
        b2.to_mut().i = 3;
    }
}
