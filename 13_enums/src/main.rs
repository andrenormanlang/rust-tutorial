// Enum is a versatile tool that is used to represent a type that can be one of several possible variants.
// Enums are defined using the enum keyword and each variant is separated by a comma.
// enum IpAddrKind {
//     V4,
//     V6,
// }
// Using enums above is very useful to represent the kind of IP address since there are only two possible values for IP address.
// V4 is for IPv4 that has 32-bit address space and V6 is for IPv6 that has 128-bit address space.

fn main() {
    enum IpAddrKind {
        V4(),
        V6(),
    }
    let _four= IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // let _home = IpAddr::V4(String::from("127.0.0.1"));
    // let _loopback = IpAddr::V6(String::from("::1"));

    // enhanced enums to store data directly in each enum variant.
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
    


    // Using structs to solve the problem of storing data with different types.
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
}
