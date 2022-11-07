一个允许重名 [Section] 解析ini的简单库

```rust
use ini_lib::ini_str;
let a = "[Interface]
Address = 10.1.1.2/24
PrivateKey = keykeykeykey
ListenPort = 51820
DNS = 8.8.8.8
[Peer]
PublicKey = keykeykeykeykeykeykeykey
Endpoint = 1.1.1.1:51820
AllowedIPs = 10.1.1.5/32
[Peer]
PublicKey = keykeykeykeykeykeykeykeykeykeykeykey
PresharedKey = keykeykeykeykeykeykeykeykeykeykeykeykeykeykeykey
Endpoint = 2.2.2.2:51820
AllowedIPs = 10.13.13.0/24
PersistentKeepalive = 25";
println!("{:#?}",ini_str!(a));
```

```rust
 use ini_lib::ini_file;
 println!("{:#?}",ini_file!("test.ini"));
```