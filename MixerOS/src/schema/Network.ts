export enum Status {
    Connected,
    Disconnected
}

export enum Type {
    LAN,
    LOOPBACK,
    WIRELESS,
    SOUNDGRID,
    AES50
}

export type NetworkPort = {
    Interface: string,
    Name: string,
    Status: Status,
    Type: Type,
    IPv4: string,
    IPv6: string,
}