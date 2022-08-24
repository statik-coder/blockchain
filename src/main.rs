use blockchainlib::Blockchain;

fn main() {
    let mut blockchain = Blockchain::bootstrap();
    blockchain.start_mining();
    println!("Blockchain: {:?}", &blockchain.get_chain());
}
