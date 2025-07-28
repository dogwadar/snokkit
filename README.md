On Linux, you’ll need to run your binary as root (or with CAP_NET_RAW) to access raw packets:

```sudo setcap cap_net_raw=eip ./your-sniffer```
