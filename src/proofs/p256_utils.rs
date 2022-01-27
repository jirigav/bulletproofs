
use curv::elliptic::curves::{p256::Secp256r1, Point, Scalar};
use curv::BigInt;
use curv::arithmetic::traits::*;

/*
Takes uniformly distributed bytes and produces p256 point with unknown logarithm

__Note:__ Temporary solution, hopefully will be removed soon
*/
const P256_PUBLIC_KEY_SIZE: usize = 33;
pub fn generate_random_point(bytes: &[u8]) -> Point<Secp256r1> {
        let compressed_point_len = P256_PUBLIC_KEY_SIZE;  //secp256k1::constants::PUBLIC_KEY_SIZE;
        let truncated = if bytes.len() > compressed_point_len - 1 {
            &bytes[0..compressed_point_len - 1]
        } else {
            &bytes
        };
        let mut buffer = [0u8; P256_PUBLIC_KEY_SIZE];
        buffer[0] = 0x2;
        buffer[1..1 + truncated.len()].copy_from_slice(truncated);
        if let Ok(point) = Point::from_bytes(&buffer) {
            return point;
        }

        let bn = BigInt::from_bytes(bytes);
        let two = BigInt::from(2);
        let bn_times_two = BigInt::mod_mul(&bn, &two, Scalar::<Secp256r1>::group_order());
        let bytes = BigInt::to_bytes(&bn_times_two);
        generate_random_point(&bytes)
}
