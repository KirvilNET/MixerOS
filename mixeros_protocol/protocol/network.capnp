@0xba549b40395b11d7;

struct MAC { # 6 bytes

}

struct IPv4 { # 6 bytes

}

struct IPv6 {

}

struct SubnetMask {

}

struct Network {
    interfaceName @0 :Text;
    status @1 :Bool;
    mac @2 :MAC; 
    union { ipv4 @3 :List(IPv4); ipv6 @4 :List(IPv6); }
    scope @5 :Text;
    subnetMask @6 :SubnetMask;
}