use quick_xml::se;
use crate::error::OnvifError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const MULTICAST_IPV4_ADDRESS: &str = "239.255.255.250";
pub const MULTICAST_PORT: u16 = 3702;

pub const PROBE_MESSAGE_TYPE: &str = "d:NetworkVideoTransmitter";

#[derive(Debug, Serialize)]
#[serde(rename = "s:Envelope")]
pub struct ProbeEnvelope {
    #[serde(rename = "@xmlns:s")]
    pub xmlns_s: String,
    #[serde(rename = "@xmlns:a")]
    pub xmlns_a: String,
    #[serde(rename = "s:Header")]
    pub header: ProbeHeader,
    #[serde(rename = "s:Body")]
    pub body: ProbeBody,
}

#[derive(Debug, Serialize)]
pub struct ProbeHeader {
    #[serde(rename = "a:Action")]
    pub action: String,
    #[serde(rename = "a:MessageID")]
    pub message_id: String,
    #[serde(rename = "a:To")]
    pub to: String,
}

#[derive(Debug, Serialize)]
pub struct ProbeBody {
    #[serde(rename = "d:Probe")]
    pub probe: Probe,
}

#[derive(Debug, Serialize)]
pub struct Probe {
    #[serde(rename = "@xmlns:d")]
    pub xmlns_d: String,
    #[serde(rename = "d:Types")]
    pub types: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Envelope")]
pub struct ProbeMatchEnvelope {
    #[serde(rename = "Header")]
    pub header: ProbeMatchHeader,
    #[serde(rename = "Body")]
    pub body: ProbeMatchBody,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ProbeMatchHeader {
    #[serde(rename = "RelatesTo")]
    pub relates_to: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ProbeMatchBody {
    #[serde(rename = "ProbeMatches")]
    pub probe_matches: ProbeMatches,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ProbeMatches {
    #[serde(rename = "ProbeMatch", default)]
    pub probe_match: Vec<ProbeMatch>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ProbeMatch {
    #[serde(rename = "Types")]
    pub types: String,
    #[serde(rename = "Scopes")]
    pub scopes: String,
    #[serde(rename = "XAddrs")]
    pub xaddrs: String,
    #[serde(rename = "MessageID")]
    pub message_id: String,
}

pub fn build_probe_message() -> Result<String, OnvifError> {
    let message_id = format!("uuid:{}", Uuid::new_v4());
    let probe = ProbeEnvelope {
        xmlns_s: "http://www.w3.org/2003/05/soap-envelope".to_string(),
        xmlns_a: "http://schemas.xmlsoap.org/ws/2004/08/addressing".to_string(),
        header: ProbeHeader {
            action: "http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe".to_string(),
            message_id: message_id.clone(),
            to: "urn:schemas-xmlsoap-org:ws:2005:04:discovery".to_string(),
        },
        body: ProbeBody {
            probe: Probe {
                xmlns_d: "http://schemas.xmlsoap.org/ws/2005/04/discovery".to_string(),
                types: PROBE_MESSAGE_TYPE.to_string(),
            },
        },
    };

    let mut xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string();
    let envelope_xml = se::to_string(&probe)?;
    xml.push_str(&envelope_xml);

    Ok(xml)
}
