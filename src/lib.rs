struct HashTable<T> {

    arr: [Vec<T>; 13], // using 13 just cause its a random prime
    
}

impl<T> HashTable<T> {

    // new method used for all generics
    pub fn new() -> HashTable<T> {
        
        HashTable { // not a great way to init, but this is just for practice so whatever
            arr: [ vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![] ],
        }
    }

}

// impl for Strings specifically (String and &str have Hash Trait which is below)
impl HashTable<String> { // using String not &str because want ownership
    pub fn insert(&mut self, val: String) { 

        let hash_num = val.return_hash(13);  // Hard coded 13 cause this is just an excerise, normally would be variable

        self.arr[hash_num].push(val);
    }


    pub fn is_str_in_hash(&self, val: &str) -> bool {

        // set found to false, get the hash, find bucket length
        let mut found = false;
        let hash_num = val.return_hash(13);
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

// Trait used for hashing
pub trait Hash {
    
    fn return_hash(&self, hash_size: usize) -> usize;
}

// impl used for my Hash that I included in this file
impl Hash for String {

    fn return_hash(&self, hash_size: usize) -> usize {
        
        let mut sum = 0;
        
        for c in self.chars() {
            sum += c as usize;
        }

        sum % hash_size 
    }

}

// impl used for hash look-up
impl Hash for &str {

    fn return_hash(&self, hash_size: usize) -> usize {
        
        let mut sum = 0;
        
        for c in self.chars() {
            sum += c as usize;
        }

        sum % hash_size 
    }

}


// Example impl of Hash for a char datatype
impl Hash for char {

    fn return_hash(&self, hash_size: usize) -> usize {

        *self as usize * 35 % hash_size  // deref self as usize, multplied by 35 to spread out chars, then moduloed
    }
}

// ! TESTS
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
