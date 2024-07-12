# Platform Management API

Host Linux内核通过一组ioctl来提供对这些API的访问，
`csv-rs` 通过`/dev/sev`节点向Host Linux内核
发送ioctl命令。使用`csv-rs`制作的二进制APP需要具备
与`/dev/sev`节点交互的权限。

目前支持Platform Management API列表：

