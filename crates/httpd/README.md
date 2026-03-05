# hlstate-httpd

Web server implementation based on the Rocket engine

> [!IMPORTANT]
> * IPv6-only servers implementation, make sure `xash3d-query` ([IPv6](https://github.com/YGGverse/xash3d-master/tree/ip6-only/query)) is installed!

``` bash
cd crates/httpd
cargo run -- -c config.toml
```