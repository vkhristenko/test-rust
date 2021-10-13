pub mod mod5;
pub mod mod6;

#[macro_use] extern crate function_name;

use function_name::named;

macro_rules! function_path {() => (concat!(
    module_path!(), "::", function_name!()
))}

#[named]
pub fn test1() {
    println!("{}", function_path!());
}

pub mod mod1 {
    #[named]
    pub fn test1() { println!("{}", function_path!()); }
}

pub fn test2() {
    mod5::test1();
    mod6::test1();
}

pub mod mod2 {
    #[named]
    pub fn test1() { println!("{}", function_path!()); }

    pub fn test2() { 
        test1(); 
        crate::test1(); 
        crate::mod1::test1(); 
        crate::mod2::test1(); 

        super::test1();
    }

    pub mod mod1 {
        pub fn test1() { 
            super::test1(); 
            super::super::test1();
        }
    }

    pub mod mod2 {
        pub struct SomeStruct {
            pub data1: Vec<u8>,
            data2: Vec<u8>
        }

        impl SomeStruct {
            pub fn build() -> SomeStruct {
                SomeStruct {
                    data1: vec![],
                    data2: vec![]
                }
            }
        }
    }

    mod mod3 {
        pub mod mod1 {
            pub mod mod1 {
                pub fn some_func() {}
            }
        }

        pub use mod1::mod1 as mod2;
    }

    fn testme() {
        //let s = mod2::SomeStruct { data1: vec![1,2,3], data2: vec![1,2,3,4] };
        let s = mod2::SomeStruct::build();
        use mod2::SomeStruct as SomeStruct;
        let s1 = SomeStruct::build();

        //mod3::mod1::some_func();
        mod3::mod1::mod1::some_func();
        mod3::mod2::some_func();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
