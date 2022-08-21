use blockchainlib::Block;

fn main() {
    let mut block = Block::new(
        0,
        vec![0, 64],
        0,
        "Genesis block".to_string(),
        0x0000ffffffffffffffffffffffffffff,
    );
    block.create_hash();
    block.mine();
    print!("Block mined successfully: {:?}", &block);
}
