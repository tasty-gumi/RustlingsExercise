// TODO: Fix the compiler error by moving the whole definition of this macro.
// 宏定义必须在首次调用之前
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
