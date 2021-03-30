struct HashTable<T> {

    arr: [Vec<T>; 13], // using 13 just cause its a random prime
    
}

impl<T> HashTable<T> {

    pub fn new() -> HashTable<T> {
        
        HashTable { // not a great way to init, but was running into issues with alloc and Copy Trait
            arr: [ vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![] ],
        }
    }


    pub fn is_in_hash(val: &T) -> bool {

        unimplemented!();
    }

}

// impl for Strings specifically
impl HashTable<String> { // using String not &str because want ownership
    pub fn insert(&mut self, val: String) { 

        let hash_num = HashTable::hash(&val);

        println!("Hash for {0} is {1}", val, hash_num);

        self.arr[hash_num].push(val);
    }

    fn hash(val: &str) -> usize {

        let mut sum = 0;
        
        for c in val.chars() {
            sum += c as usize;
        }

        sum % 13 // hard coded 13, (length of array), this is to attack my future self #TrustNoOne
    }

    pub fn is_str_in_hash(&self, val: &str) -> bool {

        // set found to false, get the hash, find bucket length
        let mut found = false;
        let hash_num = HashTable::hash(&val);
        let vec_length = self.arr[hash_num].len();
        // iter until found or at end of bucket
        for i in 0..vec_length {
            if val == self.arr[hash_num][i] {
                found = true;
            }
        }
        found
    }

}

trait hash {
    // TODO: Plan is to impl a trait for each data type and use that in the generic HashTable
}


#[cfg(test)]
mod tests 
{


    #[test]
    fn hashing_string_and_finding_it() {
        use super::*;
        let mut example: HashTable<String> = HashTable::new();
        example.insert("KitKat".to_string());

        assert!(example.is_str_in_hash("KitKat"));
    }

    #[test]
    fn hash_not_in_table() {
        use super::*;
        let empty_hash: HashTable<String> = HashTable::new();
        
        assert_eq!(empty_hash.is_str_in_hash("Coffee"), false ); // should return false if coffe not found
    }


    #[test]
    fn hashing_many_strs_and_finding() {
        use super::*;
        let mut example: HashTable<String> = HashTable::new();
        example.insert("KitKat".to_string());
        example.insert("yogurt".to_string());
        example.insert("rust".to_string());
        example.insert("lions".to_string());
        example.insert("football".to_string());
        example.insert("DnD".to_string());
        example.insert("c++".to_string());
        example.insert("iphone".to_string());
        example.insert("Many Words".to_string());
        example.insert(".,.,/.,.".to_string());
        example.insert(" ".to_string());

        assert!(example.is_str_in_hash("DnD"));
    }

}
