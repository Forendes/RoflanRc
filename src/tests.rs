use crate::rc::*;

#[test]
fn drop_test() {
    let a = Test { i: 10 };
    let x = Rc::new(a);
    {
        let _y = x.clone();
    }
}
