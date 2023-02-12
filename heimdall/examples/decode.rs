use heimdall::decode::decode_calldata;

fn main() {

    // Decode the calldata and save the results.
    println!("{:#?}", decode_calldata(CALLDATA.to_string()));

}

const CALLDATA: &str = "0xd57eafac000000000000000000000000b59419389d1fb089135a6a2c4bb32e5e8aa8b3330000000000000000000000001b84765de8b7566e4ceaf4d0fd3c5af52d3dde4f000000000000000000000000000000000000000000000f41a839dee4932bd176000000000000000000000000000000000000000000000004afd16002efcae82f0000000000000000000000001116898dda4015ed8ddefb84b6e8bc24528af2d80000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000101c599f9f0000000000000000000000000000000000000000000000000000000063eaa06d8514775c2c1663f55720b8c4291fbb33e6524316ebc6919a2d9a459811072867";