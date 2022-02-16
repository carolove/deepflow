//! Referfence `gopacket/layers/enums.go`

use std::fmt;

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// EthernetType is an enumeration of ethernet type values, and acts as a decoder
/// for any type it supports.
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum EthernetType {
    // EthernetTypeLLC is not an actual ethernet type.  It is instead a
    // placeholder we use in Ethernet frames that use the 802.3 standard of
    // srcmac|dstmac|length|LLC instead of srcmac|dstmac|ethertype.
    Llc = 0,
    Ipv4 = 0x0800,
    Arp = 0x0806,
    Ipv6 = 0x86DD,
    CiscoDiscovery = 0x2000,
    NortelDiscovery = 0x01a2,
    TransparentEthernetBridging = 0x6558,
    Dot1Q = 0x8100,
    Ppp = 0x880b,
    PppoeDiscovery = 0x8863,
    PppoeSession = 0x8864,
    MplsUnicast = 0x8847,
    MplsMulticast = 0x8848,
    Eapol = 0x888e,
    QinQ = 0x88a8,
    LinkLayerDiscovery = 0x88cc,
    EthernetCtp = 0x9000,
}

impl Default for EthernetType {
    fn default() -> Self {
        EthernetType::Llc
    }
}

impl PartialEq<u16> for EthernetType {
    fn eq(&self, other: &u16) -> bool {
        u16::from(*self).eq(other)
    }
}

impl PartialEq<EthernetType> for u16 {
    fn eq(&self, other: &EthernetType) -> bool {
        u16::from(*other).eq(self)
    }
}

// IPProtocol is an enumeration of IP protocol values, and acts as a decoder
// for any type it supports.
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum IpProtocol {
    Ipv6HopByHop = 0,
    Icmpv4 = 1,
    Igmp = 2,
    Ipv4 = 4,
    Tcp = 6,
    Udp = 17,
    Rudp = 27,
    Ipv6 = 41,
    Ipv6Routing = 43,
    Ipv6Fragment = 44,
    Gre = 47,
    Esp = 50,
    Ah = 51,
    Icmpv6 = 58,
    NoNextHeader = 59,
    Ipv6Destination = 60,
    Ospf = 89,
    Ipip = 94,
    EtherIp = 97,
    Vrrp = 112,
    Sstp = 132,
    UdpLite = 136,
    MplsInIp = 137,
}

impl Default for IpProtocol {
    fn default() -> Self {
        IpProtocol::Ipv6HopByHop
    }
}

impl PartialEq<u8> for IpProtocol {
    fn eq(&self, other: &u8) -> bool {
        u8::from(*self).eq(other)
    }
}

impl PartialEq<IpProtocol> for u8 {
    fn eq(&self, other: &IpProtocol) -> bool {
        u8::from(*other).eq(self)
    }
}

// LinkType is an enumeration of link types, and acts as a decoder for any
// link type it supports.
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum LinkType {
    // According to pcap-linktype(7) and http://www.tcpdump.org/linktypes.html
    Null = 0,
    Ethernet = 1,
    Ax25 = 3,
    TokenRing = 6,
    ArcNet = 7,
    Slip = 8,
    Ppp = 9,
    Fddi = 10,
    PppHdlc = 50,
    PppEthernet = 51,
    AtmRfc1483 = 100,
    Raw = 101,
    Chdlc = 104,
    Ieee802_11 = 105,
    Relay = 107,
    Loop = 108,
    LinuxSLL = 113,
    Talk = 114,
    PfLog = 117,
    PrismHeader = 119,
    IpOverFc = 122,
    SunAtm = 123,
    Ieee80211Radio = 127,
    ArcNetLinux = 129,
    IpOver1394 = 138,
    Mtp2Phdr = 139,
    Mtp2 = 140,
    Mtp3 = 141,
    Sccp = 142,
    Docsis = 143,
    LinuxIrda = 144,
    LinuxLapd = 177,
    LinuxUsb = 220,
    Ipv4 = 228,
    Ipv6 = 229,
}

impl PartialEq<u8> for LinkType {
    fn eq(&self, other: &u8) -> bool {
        u8::from(*self).eq(other)
    }
}

impl PartialEq<LinkType> for u8 {
    fn eq(&self, other: &LinkType) -> bool {
        u8::from(*other).eq(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapType {
    Any,
    Isp(u8),
    Tor,
    Max,
}

impl TryFrom<u16> for TapType {
    type Error = &'static str;
    fn try_from(t: u16) -> Result<TapType, Self::Error> {
        match t {
            0 => Ok(TapType::Any),
            3 => Ok(TapType::Tor),
            v if v < 256 => Ok(TapType::Isp(v as u8)),
            _ => Err("tap_type not in [0, 256)"),
        }
    }
}

impl From<TapType> for u16 {
    fn from(t: TapType) -> u16 {
        match t {
            TapType::Any => 0,
            TapType::Isp(v) => v as u16,
            TapType::Tor => 3,
            TapType::Max => 256,
        }
    }
}

impl Default for TapType {
    fn default() -> TapType {
        TapType::Any
    }
}

impl fmt::Display for TapType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TapType::Any => write!(f, "any"),
            TapType::Isp(n) => write!(f, "isp{}", n),
            TapType::Tor => write!(f, "tor"),
            TapType::Max => write!(f, "max"),
        }
    }
}

// 因为不知道Windows 的iftype 有那些，只能写一些常用的
//https://docs.microsoft.com/en-us/windows/win32/api/iptypes/ns-iptypes-ip_adapter_addresses_lh
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum IfType {
    Other = 1,
    Ethernet = 6,
    TokenRing = 9,
    Ppp = 23,
    Loopback = 24,
    Atm = 37,
    Ieee80211 = 71,
    Tunnel = 131,
    Ieee1394 = 144,
}

impl fmt::Display for IfType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IfType::Other => write!(f, "other"),
            IfType::Ethernet => write!(f, "ethernet"),
            IfType::TokenRing => write!(f, "tokenping"),
            IfType::Ppp => write!(f, "ppp"),
            IfType::Loopback => write!(f, "loopback"),
            IfType::Atm => write!(f, "atm"),
            IfType::Ieee80211 => write!(f, "ieee80211"),
            IfType::Tunnel => write!(f, "tunnel"),
            IfType::Ieee1394 => write!(f, "ieee1394"),
        }
    }
}

#[repr(u8)]
pub enum HeaderType {
    Invalid = 0,
    Eth = 0x1,
    Arp = 0x2,
    Ipv4 = 0x20,
    Ipv4Icmp = 0x21,
    Ipv6 = 0x40,
    Ipv4Tcp = 0x80,
    Ipv4Udp = 0x81,
    Ipv6Tcp = 0xb0,
    Ipv6Udp = 0xb1,
}

#[allow(non_upper_case_globals)]
impl HeaderType {
    pub const L2: HeaderType = HeaderType::Eth;
    pub const L3: HeaderType = HeaderType::Ipv4;
    pub const L3Ipv6: HeaderType = HeaderType::Ipv6;
    pub const L4: HeaderType = HeaderType::Ipv4Tcp;
    pub const L4Ipv6: HeaderType = HeaderType::Ipv6Tcp;
}

impl Default for HeaderType {
    fn default() -> HeaderType {
        HeaderType::Invalid
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn assert_ethernet_type() {
        let eth_type = EthernetType::Ipv6;
        let ipv6: u16 = eth_type.into();
        assert_eq!(eth_type, 0x86DDu16);
        assert_eq!(0x86DDu16, eth_type);
        assert_eq!(ipv6, 0x86DDu16);
        assert_eq!(Ok(EthernetType::Arp), EthernetType::try_from(0x806u16));
    }

    #[test]
    fn assert_link_type() {
        let link_type = LinkType::Ppp;
        assert_eq!(link_type, 9);
        assert_eq!(9, link_type);
        assert_eq!(Ok(LinkType::Talk), LinkType::try_from(114u8));
    }

    #[test]
    fn assert_ip_protocol() {
        let ip = IpProtocol::Icmpv6;
        assert_eq!(ip, 58);
        assert_eq!(58, ip);
        assert_eq!(Ok(IpProtocol::Udp), IpProtocol::try_from(17u8));
    }
}
