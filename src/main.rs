use lambdaworks_math::elliptic_curve::ProjectiveArithmetic;
use lambdaworks_math::elliptic_curve::weierstrass::Point;

fn main() {
    let secret_key: u64 = 0x6C616D6264617370;
    let g = lambdaworks_math::elliptic_curve::weierstrass::G1Projective::generator();
    let public_key = g.scalar_mul(secret_key.into());

    println!("Secret key: {:x}", secret_key);
    println!("Public key: {:?}", public_key);

    // Convert the public key to affine coordinates
    let public_key_affine = public_key.to_affine().unwrap();

    // Extract the x and y coordinates
    let x = public_key_affine.x();
    let y = public_key_affine.y();

    // Print the coordinates
    println!("Public key (x): {}", x);
    println!("Public key (y): {}", y);
}
