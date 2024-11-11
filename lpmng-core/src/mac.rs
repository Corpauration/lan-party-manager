use crate::error::Error::{FailedToExtractMac, NoMacForThisIp};
use crate::error::Result;
use futures::TryStreamExt;
use netlink_packet_route::neighbour::NeighbourAddress::Inet;
use netlink_packet_route::neighbour::NeighbourAttribute;
use rtnetlink::{Handle, IpVersion};
use std::net::Ipv4Addr;
pub struct MacHandler(Handle);

impl MacHandler {
    pub fn new() -> Result<Self> {
        let (connection, netlink, _) = rtnetlink::new_connection()?;
        tokio::task::spawn(connection);
        Ok(Self(netlink))
    }

    pub async fn get_mac_from_ip(&self, ip: Ipv4Addr) -> Result<String> {
        let neighbours = self
            .0
            .neighbours()
            .get()
            .set_family(IpVersion::V4)
            .execute()
            .try_collect::<Vec<_>>()
            .await?;

        let neighbour = neighbours
            .into_iter()
            .filter(|e| {
                e.attributes
                    .contains(&NeighbourAttribute::Destination(Inet(ip)))
            })
            .nth(0)
            .ok_or(NoMacForThisIp(ip))?;

        let mac = neighbour
            .attributes
            .into_iter()
            .find(|e| matches!(*e, NeighbourAttribute::LinkLocalAddress(_)))
            .ok_or(FailedToExtractMac)?;
        let NeighbourAttribute::LinkLocalAddress(mac) = mac else {
            Err(FailedToExtractMac)?
        };
        let mac = mac
            .into_iter()
            .map(|e| format!("{e:02x?}"))
            .collect::<Vec<_>>()
            .join(":");

        Ok(mac)
    }
}
