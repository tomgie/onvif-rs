use quick_xml::se;
use crate::error::OnvifError;
use serde::Serialize;
use uuid::Uuid;

pub const MULTICAST_IPV4_ADDRESS: &str = "239.255.255.250";
pub const MULTICAST_PORT: u16 = 3702;

#[derive(Debug, Serialize)]
#[serde(rename = "s:Envelope")]
pub struct ProbeEnvelope {
    #[serde(rename = "@xmlns:s")]
    pub xmlns_s: String,
    #[serde(rename = "@xmlns:a")]
    pub xmlns_a: String,
    #[serde(rename = "@xmlns:wsdd")]
    pub xmlns_wsdd: String,
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
    #[serde(rename = "wsdd:Probe")]
    pub probe: Probe,
}

#[derive(Debug, Serialize)]
pub struct Probe {
    #[serde(rename = "wsdd:Types")]
    pub types: String,
    #[serde(rename = "wsdd:Scopes")]
    pub scopes: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct EndpointReference {
    pub address: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ProbeMatch {
    pub endpoint_reference: EndpointReference,
    pub types: String,
    pub scopes: String,
    pub xaddrs: String,
    pub metadata_version: u32,
}

pub fn build_probe_message() -> Result<String, OnvifError> {
    let message_id = format!("urn:uuid:{}", Uuid::new_v4());
    let probe = ProbeEnvelope {
        xmlns_s: "http://www.w3.org/2003/05/soap-envelope".to_string(),
        xmlns_a: "http://schemas.xmlsoap.org/ws/2004/08/addressing".to_string(),
        xmlns_wsdd: "http://schemas.xmlsoap.org/ws/2005/04/discovery".to_string(),
        header: ProbeHeader {
            action: "http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe".to_string(),
            message_id: message_id.clone(),
            to: "urn:schemas-xmlsoap-org:ws:2005:04:discovery".to_string(),
        },
        body: ProbeBody {
            probe: Probe {
                types: "tdn:NetworkVideoTransmitter".to_string(),
                scopes: "onvif://www.onvif.org/Profile/Streaming".to_string(),
            },
        },
    };

    let mut xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string();
    let envelope_xml = se::to_string(&probe)?;
    xml.push_str(&envelope_xml);

    Ok(xml)
}
