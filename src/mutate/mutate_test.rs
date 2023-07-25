// this test file is only for debug in develop, not a true test.

#[test]
fn test_tree_structure_mutate() {
    use super::geno_mutate::mutate_tree_structure;
    use crate::blob::geno_blob_builder::BlobGeno;

    let mut geno = BlobGeno::new_rand();
    println!("{:#?}",geno);
    mutate_tree_structure(&mut geno);
    println!("\n{:#?}",geno);
}