use db_dump::{self, dependencies};
use std::collections::HashMap;
fn maketree(cratename: Option<String>, tree_level: u8) {
    /*
       ____________________________________________________________________________
    * |                                                                           |
    * | chances are, we're going to implement tree_level by having the input crate|
    * | have a tree_level of 0, and then for every other crate that appears, the  |
    * | crates that it is dependent on are equiv. of currentcrte.tree_level + 1,  |
    * | and this will be used to visually represent a tree                        |
    * |___________________________________________________________________________|

    */ 
    
    db_dump::Loader::new().dependencies(|row| {
        let crate_name = row.explicit_name;
        dependancy_tree.push(CrateInfo{ 
            cratename: crate_name,
            tree_level: tree_level + 1
        }); 
    }).load("./db-dump.tar.gz");

}

fn main() -> db_dump::Result<()> {

    struct CrateInfo {
        cratename: Option<String>,
        tree_level: u8,
    }

    let mut dependancy_tree: Vec<CrateInfo> = Vec::new(); //u32: crate ID, u8: level in the tree 


    if let Some(last_crate) = dependancy_tree.last() {
        let stack_cratename = last_crate.cratename.clone();
        let stack_treelevel = last_crate.tree_level;
        maketree(stack_cratename, stack_treelevel);
    }
    Ok(())
}
