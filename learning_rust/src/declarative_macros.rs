

#[cfg(test)]
mod test {

    macro_rules! expression_macro {
        ($x: expr) => {
            format!("Exxpression result is: {}", $x)
        };
    }

    macro_rules! type_macro {
        ($x: ty) => {
            match stringify!($x) {
                "i32" => "i32 type".to_string(),
                _ => "Not an i32".to_string(),
            }            
        };
    }


    #[test]
    fn tests_declarative_macro(){
        let macro_output: String = expression_macro!(3-3);
        dbg!(macro_output);

        let macro_output: String = type_macro!(str);
        dbg!(macro_output);
        
    }

}