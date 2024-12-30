// TODO: Fix the compiler error without taking the macro definition out of this
// module.
 mod macros {
    //使用导出宏定义属性让写在mod中的宏被外界访问。
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
   my_macro!();
}
