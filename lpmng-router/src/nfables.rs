use crate::error::Error::{CommandError, NoResult};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;
use tracing::debug;

pub struct Nftables {
    table: String,
    set: String,
}

impl Nftables {
    pub fn new(table: String, set: String) -> Self {
        Self { table, set }
    }

    pub fn get_items_in_set(&self) -> Result<Vec<String>> {
        let data = NfRoot {
            nftables: vec![NfFact::List(NfList::Set {
                family: "inet".to_string(),
                table: self.table.to_string(),
                name: self.set.to_string(),
            })],
        };

        let res = serde_json::to_string(&data)?;
        debug!("{res}");

        let res = Self::run_command(vec![res.as_str()])?;
        let res = serde_json::from_slice::<NfRoot>(&res)?;

        let fact = res
            .nftables
            .into_iter()
            .find(|e| matches!(e, NfFact::Set { .. }))
            .ok_or(NoResult)?;
        if let NfFact::Set {
            elem: Some(elem), ..
        } = fact
        {
            Ok(elem)
        } else {
            Ok(vec![])
        }
    }

    pub fn add_items_in_set(&self, items: Vec<String>) -> Result<()> {
        let data = NfRoot {
            nftables: vec![NfFact::Add(NfType::Element {
                family: "inet".to_string(),
                table: self.table.to_string(),
                name: self.set.to_string(),
                elem: Some(items),
            })],
        };

        let res = serde_json::to_string(&data)?;
        debug!("{res}");

        Self::run_command(vec![res.as_str()])?;
        Ok(())
    }

    pub fn delete_items_in_set(&self, items: Vec<String>) -> Result<()> {
        let data = NfRoot {
            nftables: vec![NfFact::Delete(NfType::Element {
                family: "inet".to_string(),
                table: self.table.to_string(),
                name: self.set.to_string(),
                elem: Some(items),
            })],
        };

        let res = serde_json::to_string(&data)?;
        debug!("{res}");

        Self::run_command(vec![res.as_str()])?;
        Ok(())
    }

    pub fn flush_set(&self) -> Result<()> {
        let items = self.get_items_in_set()?;
        self.delete_items_in_set(items)
    }

    fn run_command(args: Vec<&str>) -> Result<Vec<u8>> {
        let res = Command::new("nft")
            .args(["-j"].into_iter().chain(args.into_iter()))
            .output()?;
        if !res.status.success() {
            Err(CommandError(
                res.status.code(),
                String::from_utf8_lossy(&res.stderr).to_string(),
            ))
        } else {
            Ok(res.stdout)
        }
    }
}

#[derive(Deserialize, Serialize)]
struct NfRoot {
    nftables: Vec<NfFact>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum NfFact {
    Metainfo {
        version: String,
        release_name: String,
        json_schema_version: usize,
    },
    Set {
        family: String,
        name: String,
        table: String,
        r#type: String,
        handle: usize,
        elem: Option<Vec<String>>,
    },
    Add(NfType),
    Delete(NfType),
    List(NfList),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum NfType {
    Element {
        family: String,
        table: String,
        name: String,
        elem: Option<Vec<String>>,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum NfList {
    Set {
        family: String,
        table: String,
        name: String,
    },
}
