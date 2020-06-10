mod sphere;
pub use sphere::*;
mod indices;
pub use indices::*;
mod cube;
pub use cube::*;
mod floor;
pub use floor::*;
mod region;
pub use region::*;
mod ramps;
pub use ramps::*;
pub mod rex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapidx_idxmap() {
        let (x, y, z) = (12, 19, 11);
        let idx = mapidx(x, y, z);
        let (nx, ny, nz) = idxmap(idx);
        assert_eq!(x, nx);
        assert_eq!(y, ny);
        assert_eq!(z, nz);
    }

    #[test]
    fn test_mapidx() {
        assert_eq!(mapidx(1usize, 0usize, 0usize), 1usize);
        assert_eq!(mapidx(2usize, 0usize, 0usize), 2usize);
    }
}
