
pub mod opnorm;
pub mod qr;
pub mod svd;
pub mod solve;
pub mod cholesky;
pub mod eigh;
pub mod triangular;

pub use self::opnorm::*;
pub use self::qr::*;
pub use self::svd::*;
pub use self::solve::*;
pub use self::cholesky::*;
pub use self::eigh::*;
pub use self::triangular::*;

use super::error::*;

trait_alias!(LapackScalar: OperatorNorm_,
             QR_,
             SVD_,
             Solve_,
             Cholesky_,
             Eigh_,
             Triangular_);

pub fn into_result<T>(info: i32, val: T) -> Result<T> {
    if info == 0 {
        Ok(val)
    } else {
        Err(LapackError::new(info).into())
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UPLO {
    Upper = b'U',
    Lower = b'L',
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Transpose {
    No = b'N',
    Transpose = b'T',
    Hermite = b'C',
}
