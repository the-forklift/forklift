use db_dump::{self, dependencies};
use std::collections::HashMap;

fn main() -> db_dump::Result<()> {
    let tree_level = 0; //this is a plcaeholder
    /*
    *  ____________________________________________________________________________
    * |                                                                           |
    * | chances are, we're going to implement tree_level by having the input crate|
    * | have a tree_level of 0, and then for every other crate that appears, the  |
    * | crates that it is dependent on are equiv. of currentcrte.tree_level + 1,  |
    * | and this will be used to visually represent a tree                        |
    * |___________________________________________________________________________|
    */ 
    let mut dependancy_tree: HashMap<Option<String>, u8> = HashMap::new(); //u32: crate ID, u8: level in the tree 
    db_dump::Loader::new().dependencies(|row| {
        let crate_name = row.explicit_name;
    
        dependancy_tree.insert(crate_name, tree_level); 
    }).load("./db-dump.tar.gz")?;

    Ok(())
}
