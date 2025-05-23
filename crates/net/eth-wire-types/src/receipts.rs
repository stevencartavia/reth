//! Implements the `GetReceipts` and `Receipts` message types.

use alloc::vec::Vec;
use alloy_consensus::{ReceiptWithBloom, RlpDecodableReceipt, RlpEncodableReceipt, TxReceipt};
use alloy_primitives::B256;
use alloy_rlp::{RlpDecodableWrapper, RlpEncodableWrapper};
use reth_codecs_derive::add_arbitrary_tests;
use reth_ethereum_primitives::Receipt;

/// A request for transaction receipts from the given block hashes.
#[derive(Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[add_arbitrary_tests(rlp)]
pub struct GetReceipts(
    /// The block hashes to request receipts for.
    pub Vec<B256>,
);

/// The response to [`GetReceipts`], containing receipt lists that correspond to each block
/// requested.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[add_arbitrary_tests(rlp)]
pub struct Receipts<T = Receipt>(
    /// Each receipt hash should correspond to a block hash in the request.
    pub Vec<Vec<ReceiptWithBloom<T>>>,
);

impl<T: RlpEncodableReceipt> alloy_rlp::Encodable for Receipts<T> {
    #[inline]
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        self.0.encode(out)
    }
    #[inline]
    fn length(&self) -> usize {
        self.0.length()
    }
}

impl<T: RlpDecodableReceipt> alloy_rlp::Decodable for Receipts<T> {
    #[inline]
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        alloy_rlp::Decodable::decode(buf).map(Self)
    }
}

/// Eth/69 receipt response type that removes bloom filters from the protocol.
///
/// This is effectively a subset of [`Receipts`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[add_arbitrary_tests(rlp)]
pub struct Receipts69<T = Receipt>(pub Vec<Vec<T>>);

impl<T: RlpEncodableReceipt + alloy_rlp::Encodable> alloy_rlp::Encodable for Receipts69<T> {
    #[inline]
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        self.0.encode(out)
    }
    #[inline]
    fn length(&self) -> usize {
        self.0.length()
    }
}

impl<T: RlpDecodableReceipt + alloy_rlp::Decodable> alloy_rlp::Decodable for Receipts69<T> {
    #[inline]
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        alloy_rlp::Decodable::decode(buf).map(Self)
    }
}

impl<T: TxReceipt> Receipts69<T> {
    /// Encodes all receipts with the bloom filter.
    ///
    /// Note: This is an expensive operation that recalculates the bloom for each receipt.
    pub fn into_with_bloom(self) -> Receipts<T> {
        Receipts(
            self.0
                .into_iter()
                .map(|receipts| receipts.into_iter().map(|r| r.into_with_bloom()).collect())
                .collect(),
        )
    }
}

impl<T: TxReceipt> From<Receipts69<T>> for Receipts<T> {
    fn from(receipts: Receipts69<T>) -> Self {
        receipts.into_with_bloom()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{message::RequestPair, GetReceipts, Receipts};
    use alloy_consensus::TxType;
    use alloy_primitives::{hex, Log};
    use alloy_rlp::{Decodable, Encodable};

    #[test]
    fn roundtrip_eip1559() {
        let receipts = Receipts(vec![vec![ReceiptWithBloom {
            receipt: Receipt { tx_type: TxType::Eip1559, ..Default::default() },
            logs_bloom: Default::default(),
        }]]);

        let mut out = vec![];
        receipts.encode(&mut out);

        let mut out = out.as_slice();
        let decoded = Receipts::decode(&mut out).unwrap();

        assert_eq!(receipts, decoded);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn encode_get_receipts() {
        let expected = hex!(
            "f847820457f842a000000000000000000000000000000000000000000000000000000000deadc0dea000000000000000000000000000000000000000000000000000000000feedbeef"
        );
        let mut data = vec![];
        let request = RequestPair {
            request_id: 1111,
            message: GetReceipts(vec![
                hex!("00000000000000000000000000000000000000000000000000000000deadc0de").into(),
                hex!("00000000000000000000000000000000000000000000000000000000feedbeef").into(),
            ]),
        };
        request.encode(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn decode_get_receipts() {
        let data = hex!(
            "f847820457f842a000000000000000000000000000000000000000000000000000000000deadc0dea000000000000000000000000000000000000000000000000000000000feedbeef"
        );
        let request = RequestPair::<GetReceipts>::decode(&mut &data[..]).unwrap();
        assert_eq!(
            request,
            RequestPair {
                request_id: 1111,
                message: GetReceipts(vec![
                    hex!("00000000000000000000000000000000000000000000000000000000deadc0de").into(),
                    hex!("00000000000000000000000000000000000000000000000000000000feedbeef").into(),
                ]),
            }
        );
    }

    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    #[test]
    fn encode_receipts() {
        let expected = hex!(
            "f90172820457f9016cf90169f901668001b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f85ff85d940000000000000000000000000000000000000011f842a0000000000000000000000000000000000000000000000000000000000000deada0000000000000000000000000000000000000000000000000000000000000beef830100ff"
        );
        let mut data = vec![];
        let request = RequestPair {
            request_id: 1111,
            message: Receipts(vec![vec![
                ReceiptWithBloom {
                    receipt: Receipt {
                        tx_type: TxType::Legacy,
                        cumulative_gas_used: 0x1u64,
                        logs: vec![
                            Log::new_unchecked(
                                hex!("0000000000000000000000000000000000000011").into(),
                                vec![
                                    hex!("000000000000000000000000000000000000000000000000000000000000dead").into(),
                                    hex!("000000000000000000000000000000000000000000000000000000000000beef").into(),
                                ],
                                hex!("0100ff")[..].into(),
                            ),
                        ],
                        success: false,
                    },
                    logs_bloom: hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").into(),
                },
            ]]),
        };
        request.encode(&mut data);
        assert_eq!(data, expected);
    }

    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    #[test]
    fn decode_receipts() {
        let data = hex!(
            "f90172820457f9016cf90169f901668001b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f85ff85d940000000000000000000000000000000000000011f842a0000000000000000000000000000000000000000000000000000000000000deada0000000000000000000000000000000000000000000000000000000000000beef830100ff"
        );
        let request = RequestPair::<Receipts>::decode(&mut &data[..]).unwrap();
        assert_eq!(
            request,
            RequestPair {
                request_id: 1111,
                message: Receipts(vec![
                    vec![
                        ReceiptWithBloom {
                            receipt: Receipt {
                                tx_type: TxType::Legacy,
                                cumulative_gas_used: 0x1u64,
                                logs: vec![
                                    Log::new_unchecked(
                                        hex!("0000000000000000000000000000000000000011").into(),
                                        vec![
                                            hex!("000000000000000000000000000000000000000000000000000000000000dead").into(),
                                            hex!("000000000000000000000000000000000000000000000000000000000000beef").into(),
                                        ],
                                        hex!("0100ff")[..].into(),
                                    ),
                                ],
                                success: false,
                            },
                            logs_bloom: hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").into(),
                        },
                    ],
                ]),
            }
        );
    }
}
