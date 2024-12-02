use std::cell::RefCell;

thread_local! {
    static CHAT: RefCell<Vec<String>> = RefCell::new(Vec::new());
}


#[ic_cdk::query]
fn get_chat() -> Vec<String> {
    CHAT.with(|chat| chat.borrow().clone())
}

#[ic_cdk::update]
fn add_msg(new_msg: String) {
    CHAT.with(|chat| chat.borrow_mut().push(new_msg))
}

// new_chat: String
// chat.borrow_mut(): &mut String
// &mut String != String ???
// *(&mut String) == String



//COUNTER.with(|count| *count.borrow_mut() = n);
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
