use crate::blob::geno_blob_builder::BlobGeno;

use super::geno_mutate::mutate_tree_structure;

#[test]
fn test_tree_structure_mutate() {
    let mut geno = BlobGeno::new_rand();
    println!("{:#?}",geno);
    mutate_tree_structure(&mut geno);
    println!("\n{:#?}",geno);
}