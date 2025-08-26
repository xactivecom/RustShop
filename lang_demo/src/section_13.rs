///////////////////////////////
// Course Section 13
///////////////////////////////

use std::rc::Rc;
use std::cell::RefCell;

struct Flagger {
    is_ok: RefCell<bool>,
}


struct RcFlagger {
    is_ok: Rc<RefCell<bool>>,
}

pub fn run_lesson() {
    println!("\nSection 13:");

    // Create box pointer to a tuple on the stack
    // It allocates values on the heap, but the pointer to the heap data remains on the stack.
    // It performs compile-time borrow checking.
    let order = (12, "eggs"); // created on stack
    let p_order_1 = Box::new(order); // created on the heap
    let p_order_2 = Box::new(order); // created on the heap
    println!("{:?} {:?}", p_order_1, p_order_2);

    // Box pointer must be dereferenced to compare value
    assert_eq!(order, *p_order_1);
    assert_eq!(order, *p_order_2);

    // An Rc (reference counted) pointer is stored on the heap and allows multiple owners
    // Atomic rference counts are used in multi-threaded situations
    let r_1 = Rc::new(String::from("Smiths"));
    let r_2 = r_1.clone();
    let r_3 = r_2.clone();
    println!("data:{}, contains:{}", r_3, r_3.contains("mith"));
    println!("count:{}", Rc::strong_count(&r_1));

    // Create reference cell, which still does compile-time borrow checking.
    // Method borrow() returns Ref<T>
    // Method borrow_mut() returns RefMut<T>
    let flag = Flagger{ is_ok: RefCell::new(true) };
    let flag_ref = flag.is_ok.borrow();
    println!("is_ok ref:{}", flag_ref);

    // Create mutable reference cell and modify value
    // A mutable Rc-pointer enforces borrowing rules at runtime, not the usual compile-time borrow checking.
    let rc_flag = RcFlagger{ is_ok: Rc::new(RefCell::new(true)) };
    let mut flag_mut_ref = rc_flag.is_ok.borrow_mut();
    *flag_mut_ref = false;
    println!("is_ok mut ref:{}", flag_mut_ref);

    // Assignment 13
    let s_v = 3.14159;
    let h_v = Box::new(10.2);
    println!("product:{}", s_v * *h_v);

    let s = String::from("algebra");
    let rc_1 = Rc::new(s);
    println!("count:{}", Rc::strong_count(&rc_1));
    let rc_2 = Rc::clone(&rc_1);
    println!("count:{}", Rc::strong_count(&rc_2));

}