use std::collections::{HashMap, HashSet};

#[cfg(test)]

mod test{
    use super::*;

    #[test]
    fn tests_hashmap(){
        let person_1: &str = "Andrew";
        let person_2: &str = "Diana";

        let mut results_hm: HashMap<&str,u32>= HashMap::new();
        results_hm.insert(person_1, 10);
        results_hm.insert(person_2, 8);

        let test_results: Option<&u32> = results_hm.get(person_1);
        dbg!(test_results, test_results.unwrap());
        
        if results_hm.contains_key("Andrew") {
            dbg!("Well done Andrew");
        }
    }

    #[test]
    fn tests_hashset(){
        let mut name_hs: HashSet<&str> = HashSet::new();
        name_hs.insert("Brian");
        name_hs.insert("Jobs");
        name_hs.insert("Johnny");

        if name_hs.contains("Johnny"){
            dbg!("Hi Johnny");
        }
    }
}