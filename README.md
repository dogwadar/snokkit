# snokkit
## Unfinished & Under Active Development

## Install

1. Download and navigate to root folder
2. Build with ```cargo build --release```
3. Navigate to ```/target/release/```

On Linux, you’ll need to run your binary as root (or with CAP_NET_RAW) to access raw packets:

```sudo setcap cap_net_raw=eip ./snokkit```

## Usage

Usage: ./snokkit [COMMAND] [FLAG]

Commands:
<ul>
    <li><b>list</b> - List available interfaces</li>
    <li><b>capture</b> - Start capturing packets</li>
    <li><b>help</b> - Print this message or the help of the given subcommand(s)</li>
</ul>

Options:
<ul>
    <li><b>-h, --help</b> - Print help</li>
    <li><b>-V, --version</b> - Print version</li>
</ul>


## Example

![Example](img/example.png)
