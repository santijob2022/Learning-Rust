
#[cfg(test)]
mod test {

    macro_rules! my_vec{
        ( $($x:expr),+ ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )+
                temp_vec
            }           
        }
    }


    #[test]
    fn tests_macro_my_vec(){
        let x: Vec<i32> = vec!();
        let mut y: Vec<i32> = my_vec!(1);
        y.push(3);
        dbg!(y);        
    }

}