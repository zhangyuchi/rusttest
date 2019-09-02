use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Weak;

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
    // 其他字段
}

struct Gadget {
    id: i32,
    owner: Arc<Owner>,
    // 其他字段
}

pub fn thread_ref() {
    // 创建一个可计数的Owner。
    // 注意我们将gadgets赋给了Owner。
    // 也就是在这个结构体里， gadget_owner包含gadets
    let gadget_owner: Arc<Owner> = Arc::new(Owner {
        name: "Gadget Man".to_string(),
        gadgets: RefCell::new(Vec::new()),
    });

    // 首先，我们创建两个gadget，他们分别持有 gadget_owner 的一个引用。
    let gadget1 = Arc::new(Gadget {
        id: 1,
        owner: gadget_owner.clone(),
    });
    let gadget2 = Arc::new(Gadget {
        id: 2,
        owner: gadget_owner.clone(),
    });

    // 我们将从gadget_owner的gadgets字段中持有其可变引用
    // 然后将两个gadget的Weak引用传给owner。
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Arc::downgrade(&gadget1));
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Arc::downgrade(&gadget2));

    // 遍历 gadget_owner的gadgets字段
    for gadget_opt in gadget_owner.gadgets.borrow().iter() {
        let gadget = gadget_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }
}
