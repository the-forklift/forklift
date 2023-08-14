use db_dump;

struct CrateInfo {
    cratename: Option<String>,
    tree_level: u8,
}
fn maketree(crate_name: Option<String>, tree_level: u8, stack: &mut Vec<CrateInfo>) {
    /*
       ____________________________________________________________________________
    * |                                                                           |
    * | chances are, we're going to implement tree_level by having the input crate|
    * | have a tree_level of 0, and then for every other crate that appears, the  |
    * | crates that it is dependent on are equiv. of currentcrte.tree_level + 1,  |
    * | and this will be used to visually represent a tree                        |
    * |___________________________________________________________________________|

    */ 
    
    let _ = db_dump::Loader::new().dependencies(|row| {
        stack.push(CrateInfo{ 
            cratename: row.explicit_name,
            tree_level: tree_level + 1,
        }); 
    }).load("./db-dump.tar.gz");
    
    while let Some(cratestack) = stack.pop() {
        maketree(cratestack.cratename, cratestack.tree_level, stack);
    }

}

fn main() -> db_dump::Result<()> {


    let mut dependancy_tree: Vec<CrateInfo> = Vec::new(); //u32: crate ID, u8: level in the tree 
    //____________________________________________________________________________________
    //need to make something which takes user input and pushes it into the dependancy tree|
    //____________________________________________________________________________________

    if let Some(last_crate) = dependancy_tree.last() {
        let stack_cratename = last_crate.cratename.clone();
        let stack_treelevel = last_crate.tree_level;
        maketree(stack_cratename, stack_treelevel, &mut dependancy_tree);
    }
    Ok(())
}
