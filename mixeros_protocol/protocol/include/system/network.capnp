@0xba549b40395b11d7;

struct MAC { # 6 bytes OUI0::OUI1::OUI2::NIC0::NIC1::NIC2
    oui0 @0 :UInt8;
    oui1 @1 :UInt8;
    oui2 @2 :UInt8;
    nic0 @3 :UInt8;
    nic1 @4 :UInt8;
    nic2 @5 :UInt8;
}

struct IPv4 { # 6 bytes
    group0 @0 :UInt8; 
    group1 @1 :UInt8;
    group2 @2 :UInt8;
    group3 @3 :UInt8;
}

struct IPv6 {
    group0 @0 :UInt16; 
    group1 @1 :UInt16;
    group2 @2 :UInt16;
    group3 @3 :UInt16;
    group4 @4 :UInt16; 
    group5 @5 :UInt16;
    group6 @6 :UInt16;
    group7 @7 :UInt16;
}

struct SubnetMask {
    group0 @0 :UInt8; 
    group1 @1 :UInt8;
    group2 @2 :UInt8;
    group3 @3 :UInt8;
}

struct Interface {
    # Name of the interface
    name @0 :Text;

    # Is the interface up or down
    status @1 :Bool;

    # Mac address of the interface
    mac @2 :MAC;

    # IPv4 addresses of the interface 
    ipv4 @3 :List(IPv4);

    # IPv6 addresses of the interface
    ipv6 @4 :List(IPv6);

    # Scope of the interface
    scope @5 :Text;

    # SubnetMask
    subnetMask @6 :SubnetMask;
}