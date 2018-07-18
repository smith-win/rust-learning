/* I'm going to create a map of vecs
 * 
 */

// use is is akin to Java's "import"
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::hash_map::Entry::{Occupied, Vacant};


fn main() {
    println!("Map of vecs");


    /* create a new map of int 32 
     * think it needs a mut */
    let mut map:HashMap<i32, Vec<String>> = HashMap::new();

    /* TODO: i is i32, what is j?  is it i32 */
    for i  in 0_i32 .. 100_i32 {
        for j in 1 .. 5 {

            /* TODO: 2 types of Strings in RUST, which type is appropriate, 
             * which is immutable */
            let  s:String = format!("STR-{}-{}", i, j);
            println!("S {}", s);

            /* ensure map contains a vec 
             * TODO: next challenge is to do the lazy map in re-usable manner */
            let e:Entry<i32, Vec<String>> = map.entry(i);
            match e {
                Occupied(mut oe) => oe.get_mut().push(s),
                Vacant(ve) => {ve.insert(vec!(s) ); ()},
            }
        }
    }

    println!("Map size: {}", map.len());
    let some_int = 33_i32;
    println!("Map [{}] size: {}", some_int, map.get(&some_int).unwrap().len());

    // iterate ?
    for val in map.get(&some_int).unwrap().iter() {
        println!("{} Found string {}", &some_int, val);
    }

}
