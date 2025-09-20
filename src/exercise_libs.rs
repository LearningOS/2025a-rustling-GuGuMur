#![allow(dead_code, unused_variables, unused_mut, unused_imports, unreachable_code, unused_attributes, non_snake_case, unexpected_cfgs, unused_must_use, path_statements, mismatched_lifetime_syntaxes)]

#[path = "../exercises"]
pub mod exercises {    
    pub mod variables {
        pub mod variables1;
        pub mod variables2;
        pub mod variables3;
        pub mod variables4;
        pub mod variables5;
        pub mod variables6;
    }
    
    pub mod functions {
        pub mod functions1;
        pub mod functions2;
        pub mod functions3;
        pub mod functions4;
        pub mod functions5;
    }
    
    #[path = "if"]
    pub mod if_statements {
        pub mod if1;
        pub mod if2;
        pub mod if3;
    }
    
    pub mod primitive_types {
        pub mod primitive_types1;
        pub mod primitive_types2;
        pub mod primitive_types3;
        pub mod primitive_types4;
        pub mod primitive_types5;
        pub mod primitive_types6;
    }
    
    pub mod vecs {
        pub mod vecs1;
        pub mod vecs2;
    }
    
    pub mod move_semantics {
        pub mod move_semantics1;
        pub mod move_semantics2;
        pub mod move_semantics3;
        pub mod move_semantics4;
        pub mod move_semantics5;
        pub mod move_semantics6;
    }
    
    pub mod structs {
        pub mod structs1;
        pub mod structs2;
        pub mod structs3;
    }
    
    pub mod enums {
        pub mod enums1;
        pub mod enums2;
        pub mod enums3;
    }
    
    pub mod strings {
        pub mod strings1;
        pub mod strings2;
        pub mod strings3;
        pub mod strings4;
    }
    
    pub mod modules {
        pub mod modules1;
        pub mod modules2;
        pub mod modules3;
    }
    
    pub mod hashmaps {
        pub mod hashmaps1;
        pub mod hashmaps2;
        pub mod hashmaps3;
    }
    
    pub mod options {
        pub mod options1;
        pub mod options2;
        pub mod options3;
    }
    
    pub mod error_handling {
        pub mod errors1;
        pub mod errors2;
        pub mod errors3;
        pub mod errors4;
        pub mod errors5;
        pub mod errors6;
    }
    
    pub mod generics {
        pub mod generics1;
        pub mod generics2;
    }
    
    pub mod traits {
        pub mod traits1;
        pub mod traits2;
        pub mod traits3;
        pub mod traits4;
        pub mod traits5;
    }
    
    pub mod tests {
        pub mod tests1;
        pub mod tests2;
        pub mod tests3;
        pub mod tests4;
        pub mod tests5;
        pub mod tests6;
        pub mod tests7;
        pub mod tests8;
        pub mod tests9;
    }
    
    pub mod lifetimes {
        pub mod lifetimes1;
        pub mod lifetimes2;
        pub mod lifetimes3;
    }
    
    pub mod iterators {
        pub mod iterators1;
        pub mod iterators2;
        pub mod iterators3;
        pub mod iterators4;
        pub mod iterators5;
    }
    
    pub mod threads {
        pub mod threads1;
        pub mod threads2;
        pub mod threads3;
    }
    
    pub mod smart_pointers {
        pub mod arc1;
        pub mod box1;
        pub mod cow1;
        pub mod rc1;
    }
    
    pub mod macros {
        pub mod macros1;
        pub mod macros2;
        pub mod macros3;
        pub mod macros4;
    }
    
    pub mod clippy {
        pub mod clippy1;
        pub mod clippy2;
        pub mod clippy3;
    }
    
    pub mod conversions {
        pub mod as_ref_mut;
        pub mod from_into;
        pub mod from_str;
        pub mod try_from_into;
        pub mod using_as;
    }
    
    pub mod algorithm {
        pub mod algorithm1;
        pub mod algorithm2;
        pub mod algorithm3;
        pub mod algorithm4;
        pub mod algorithm5;
        pub mod algorithm6;
        pub mod algorithm7;
        pub mod algorithm8;
        pub mod algorithm9;
        pub mod algorithm10;
    }
    
    pub mod intro {
        pub mod intro1;
        pub mod intro2;
    }
    
    #[path = "quiz1.rs"]
    pub mod quiz1;
    #[path = "quiz2.rs"]
    pub mod quiz2;
    #[path = "quiz3.rs"]
    pub mod quiz3;
}
