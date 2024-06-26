use std::cell::RefCell;


thread_local!{
    static WPISY: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String, surname: i8 ) -> String {
    format!("Hello, {} {}!", name,surname)
}

#[ic_cdk::update]
fn dodaj_wpis(wpis: String){
    WPISY.with(|wpisy: &RefCell<Vec<String>>|{
        wpisy.borrow_mut().push(wpis)
    })

}

#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<String> {
    WPISY.with(|wpisy: &RefCell<Vec<String>>|{
        wpisy.borrow_mut().clone()
    })
}