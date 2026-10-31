//! Substreams module extracting native SOL transfers from Solana blocks.

#[allow(unused_imports)]
mod pb;

use pb::sol::transfers::v1::{Transfer, Transfers};
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

/// The System Program id, `11111111111111111111111111111111` in base58.
const SYSTEM_PROGRAM_ID: [u8; 32] = [0; 32];

/// Index of the `Transfer` variant in the System Program instruction enum.
const TRANSFER_DISCRIMINATOR: u32 = 2;

/// Emits one `Transfer` per System Program transfer instruction, including inner
/// (CPI) instructions, across all successful transactions in the block.
#[substreams::handlers::map]
fn map_transfers(block: Block) -> Result<Transfers, Error> {
    let transfers = block
        .walk_instructions()
        .filter(|ix| ix.program_id().0.as_slice() == SYSTEM_PROGRAM_ID)
        .filter_map(|ix| {
            let amount = parse_transfer_amount(ix.data())?;
            let accounts = ix.accounts();
            let (from, to) = (accounts.first()?, accounts.get(1)?);
            Some(Transfer {
                from: from.to_string(),
                to: to.to_string(),
                amount,
            })
        })
        .collect();

    Ok(Transfers { transfers })
}

/// Returns the lamports of a System Program `Transfer` instruction, or `None` for
/// any other instruction. Layout: `u32` LE discriminator, then `u64` LE lamports.
fn parse_transfer_amount(data: &[u8]) -> Option<u64> {
    let (discriminator, rest) = data.split_first_chunk::<4>()?;
    if u32::from_le_bytes(*discriminator) != TRANSFER_DISCRIMINATOR {
        return None;
    }
    let (lamports, _) = rest.split_first_chunk::<8>()?;
    Some(u64::from_le_bytes(*lamports))
}

#[cfg(test)]
mod tests {
    use super::parse_transfer_amount;

    #[test]
    fn parses_transfer() {
        let mut data = 2u32.to_le_bytes().to_vec();
        data.extend_from_slice(&325_253u64.to_le_bytes());
        assert_eq!(parse_transfer_amount(&data), Some(325_253));
    }

    #[test]
    fn ignores_other_instructions() {
        let mut data = 0u32.to_le_bytes().to_vec(); // CreateAccount
        data.extend_from_slice(&[0; 48]);
        assert_eq!(parse_transfer_amount(&data), None);
    }

    #[test]
    fn rejects_truncated_data() {
        assert_eq!(parse_transfer_amount(&[2, 0, 0, 0, 1]), None);
        assert_eq!(parse_transfer_amount(&[]), None);
    }
}
