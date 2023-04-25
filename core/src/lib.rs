// use tfhe::{shortint::prelude::*, FheUint3};
use concrete::prelude::*;
use concrete::{generate_keys, set_server_key, ConfigBuilder};
pub fn do_some_fhe(input1: u64, input2: u64) -> u64 {
    let config = ConfigBuilder::all_disabled().enable_default_uint8().build();

    let (client_key, server_key) = generate_keys(config);

    set_server_key(server_key);

    let clear_a = 27u8;
    let clear_b = 128u8;

    let a = FheUint8::encrypt(clear_a, &client_key);
    let b = FheUint8::encrypt(clear_b, &client_key);

    let result = a + b;

    let decrypted_result: u8 = result.decrypt(&client_key);

    let clear_result = clear_a + clear_b;

    assert_eq!(decrypted_result, clear_result);

    // Generate a set of client/server keys
    // // with 2 bits of message and 2 bits of carry
    // let (client_key, server_key) = gen_keys(PARAM_MESSAGE_2_CARRY_2);

    // let msg1 = input1;
    // let msg2 = input2;

    // // Encrypt two messages using the (private) client key:
    // let ct_1 = client_key.encrypt(msg1);
    // let ct_2 = client_key.encrypt(msg2);

    // // Homomorphically compute an addition
    // let ct_add = server_key.unchecked_add(&ct_1, &ct_2);

    // // // Decrypt the ciphertext using the (private) client key
    // client_key.decrypt(&ct_add)
}

#[cfg(test)]
mod test {
    use crate::do_some_fhe;

    #[test]
    fn test() {
        // TODO
        let sum = do_some_fhe(1, 2);
        assert_eq!(sum, 3)
    }
}
