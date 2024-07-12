# Guest API

Guest Linux内核通过一组ioctl来提供对这些API的访问，
`csv-rs` 通过`/dev/csv-guest`节点向Guest Linux内核
发送ioctl命令。使用`csv-rs`制作的二进制APP需要具备
与`/dev/csv-guest`节点交互的权限。

目前支持Guest API列表：
1. GET_REPORT

### GET_REPORT
该命令以用户自定义的report data作为输入，向HYGON Secure
Processor请求Attestation Report。
