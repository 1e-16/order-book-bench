// use std::ops::AddAssign;
//
// struct Mask<'a, T> {
//     meta: &'a [T],
// }
//
// impl<'a, T> Mask<'a, T>
//     where
//         T: 'a + Into<usize> + Clone, usize: AddAssign<T>
// {
//     fn new(meta: &'a [T]) -> Self {
//         Self { meta }
//     }
//
//     fn encode(&self, mem: &[impl Into<u64> + Clone]) -> u64 {
//         let mut encoded_value: u64 = 0;
//         let mut shift_bits: usize = 0;
//
//         for (i, &ref size) in self.meta.iter().enumerate() {
//             if i >= mem.len() { break }
//
//             let value = mem[i].clone().into();
//             encoded_value |= value << shift_bits;
//             shift_bits += size.clone();
//         }
//
//         encoded_value
//     }
//
//     fn decode(&self, v: u64) -> Vec<u64> {
//         let mut decoded_values: Vec<u64> = Vec::new();
//         let mut shift_bits: usize = 0;
//
//         for size in self.meta {
//             let mask: u64 = (1 << size.clone().into()) - 1;
//             let value: u64 = (v >> shift_bits) & mask;
//             decoded_values.push(value);
//             shift_bits += size.clone();
//         }
//
//         decoded_values
//     }
// }
//
// // struct Mask<'a, T>
// //     where
// //         T: Into<usize> + Clone,
// // {
// //     meta: &'a [T],
// // }
// //
// // impl<'a, T> Mask<'a, T>
// //     where
// //         T: From<T> + Clone,
// // {
// //     fn new(meta: &'a [T]) -> Self {
// //         Self { meta }
// //     }
// //
// //     fn encode(&self, mem: &[T]) -> u64 {
// //         let mut encoded_value: u64 = 0;
// //         let mut shift_bits: usize = 0;
// //
// //         for (i, &size) in self.meta.iter().enumerate() {
// //             if i >= mem.len() {
// //                 break;
// //             }
// //
// //             let value: usize = mem[i].clone().into();
// //             encoded_value |= (value as u64) << shift_bits;
// //             shift_bits += size.clone().into();
// //         }
// //
// //         encoded_value
// //     }
// //
// //     fn decode(&self, v: u64) -> Vec<usize> {
// //         let mut decoded_values: Vec<usize> = Vec::new();
// //         let mut shift_bits: usize = 0;
// //
// //         for &size in self.meta {
// //             let mask: u64 = (1 << size.clone().into()) - 1;
// //             let value: usize = ((v >> shift_bits) & mask) as usize;
// //             decoded_values.push(value);
// //             shift_bits += size.clone().into();
// //         }
// //
// //         decoded_values
// //     }
// // }
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_encode_and_decode() {
//         let mask = Mask::new(&[4_usize, 6, 8]);
//
//         let encoded_value = mask.encode(&[5_u64, 50, 255]);
//
//         assert_eq!(encoded_value, 0b0000000000000000000000000101001100101111111111111111111111111);
//
//         let decoded_values = mask.decode(encoded_value);
//
//         assert_eq!(decoded_values, vec![5_u64, 50, 255]);
//     }
// }