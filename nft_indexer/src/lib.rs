mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

const TRACKED_CONTRACT: [u8; 20] = hex!("b47e3cd837ddf8e4c57f05d70ab865de6e193bbb");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    Ok(contract::Events {
        assigns: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Assign::match_and_decode(log) {
                            return Some(contract::Assign {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                punk_index: event.punk_index.to_string(),
                                to: event.to,
                            });
                        }

                        None
                })
            })
            .collect(),
        punk_bid_entereds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::PunkBidEntered::match_and_decode(log) {
                            return Some(contract::PunkBidEntered {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from_address: event.from_address,
                                punk_index: event.punk_index.to_string(),
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        punk_bid_withdrawns: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::PunkBidWithdrawn::match_and_decode(log) {
                            return Some(contract::PunkBidWithdrawn {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from_address: event.from_address,
                                punk_index: event.punk_index.to_string(),
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        punk_boughts: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::PunkBought::match_and_decode(log) {
                            return Some(contract::PunkBought {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from_address: event.from_address,
                                punk_index: event.punk_index.to_string(),
                                to_address: event.to_address,
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        punk_no_longer_for_sales: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::PunkNoLongerForSale::match_and_decode(log) {
                            return Some(contract::PunkNoLongerForSale {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                punk_index: event.punk_index.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        punk_offereds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::PunkOffered::match_and_decode(log) {
                            return Some(contract::PunkOffered {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                min_value: event.min_value.to_string(),
                                punk_index: event.punk_index.to_string(),
                                to_address: event.to_address,
                            });
                        }

                        None
                })
            })
            .collect(),
        punk_transfers: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::PunkTransfer::match_and_decode(log) {
                            return Some(contract::PunkTransfer {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                punk_index: event.punk_index.to_string(),
                                to: event.to,
                            });
                        }

                        None
                })
            })
            .collect(),
        transfers: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Transfer::match_and_decode(log) {
                            return Some(contract::Transfer {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                to: event.to,
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();

    // Loop over all the abis events to create changes
    events.assigns.into_iter().for_each(|evt| {
        tables
            .create_row("assign", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("to", Hex(&evt.to).to_string());
    });
    events.punk_bid_entereds.into_iter().for_each(|evt| {
        tables
            .create_row("punk_bid_entered", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from_address", Hex(&evt.from_address).to_string())
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.punk_bid_withdrawns.into_iter().for_each(|evt| {
        tables
            .create_row("punk_bid_withdrawn", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from_address", Hex(&evt.from_address).to_string())
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.punk_boughts.into_iter().for_each(|evt| {
        tables
            .create_row("punk_bought", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from_address", Hex(&evt.from_address).to_string())
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("to_address", Hex(&evt.to_address).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.punk_no_longer_for_sales.into_iter().for_each(|evt| {
        tables
            .create_row("punk_no_longer_for_sale", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap());
    });
    events.punk_offereds.into_iter().for_each(|evt| {
        tables
            .create_row("punk_offered", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("min_value", BigDecimal::from_str(&evt.min_value).unwrap())
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("to_address", Hex(&evt.to_address).to_string());
    });
    events.punk_transfers.into_iter().for_each(|evt| {
        tables
            .create_row("punk_transfer", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("to", Hex(&evt.to).to_string());
    });
    events.transfers.into_iter().for_each(|evt| {
        tables
            .create_row("transfer", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });

    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = EntityChangesTables::new();

    // Loop over all the abis events to create changes
    events.assigns.into_iter().for_each(|evt| {
        tables
            .create_row("assign", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("to", Hex(&evt.to).to_string());
    });
    events.punk_bid_entereds.into_iter().for_each(|evt| {
        tables
            .create_row("punk_bid_entered", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from_address", Hex(&evt.from_address).to_string())
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.punk_bid_withdrawns.into_iter().for_each(|evt| {
        tables
            .create_row("punk_bid_withdrawn", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from_address", Hex(&evt.from_address).to_string())
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.punk_boughts.into_iter().for_each(|evt| {
        tables
            .create_row("punk_bought", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from_address", Hex(&evt.from_address).to_string())
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("to_address", Hex(&evt.to_address).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.punk_no_longer_for_sales.into_iter().for_each(|evt| {
        tables
            .create_row("punk_no_longer_for_sale", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap());
    });
    events.punk_offereds.into_iter().for_each(|evt| {
        tables
            .create_row("punk_offered", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("min_value", BigDecimal::from_str(&evt.min_value).unwrap())
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("to_address", Hex(&evt.to_address).to_string());
    });
    events.punk_transfers.into_iter().for_each(|evt| {
        tables
            .create_row("punk_transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("punk_index", BigDecimal::from_str(&evt.punk_index).unwrap())
            .set("to", Hex(&evt.to).to_string());
    });
    events.transfers.into_iter().for_each(|evt| {
        tables
            .create_row("transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });

    Ok(tables.to_entity_changes())
}
