use lrwn::DevAddr;
use rand::Rng;
use rand::seq::IndexedRandom;

pub fn get_random_dev_addr(prefixes: &[lrwn::DevAddrPrefix]) -> DevAddr {
    let mut rng = rand::rng();

    // Pick a random one (in case multiple prefixes are configured).
    let prefix = *prefixes.choose(&mut rng).unwrap();

    // Generate random DevAddr.
    let mut dev_addr: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut dev_addr);
    #[cfg(test)]
    {
        dev_addr = [1, 2, 3, 4];
    }
    let mut dev_addr = DevAddr::from_be_bytes(dev_addr);

    // Set DevAddr prefix.
    dev_addr.set_dev_addr_prefix(prefix);
    dev_addr
}
