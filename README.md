# 通勤服务器-客户端

使用

- [`leptos`](https://github.com/leptos-rs/leptos)编写页面

- [`trunk`](https://trunkrs.dev/)构建页面

- [`reqwest`](https://github.com/seanmonstar/reqwest)和服务器沟通

- [`serde`](https://github.com/serde-rs/serde)和[`serde_json`](https://github.com/serde-rs/json)（反）序列化数据

写的确实很垃圾（）， 不过可以从中积累（惨痛的）经验（教训）

> *服务端源码参见[此处](https://github.com/twhice/commuter-server)*

# Build&Run

## 工具链准备

首先你需要有rust工具链（`nightly`），如果你已经安装了`rustup`，可以通过以下命令安装`nightly`工具链（[`leptos`](https://github.com/leptos-rs/leptos)需要）
```bash
rustup install nightly
```

然后使用这个命令将项目的工具链设置为`nightly`工具链（如果你的全局工具链是`nighyly`，你可以跳过这一步）
```bash
rustup override set nightly
```

然后添加`wasm32-unknown-unknown`编译目标（[`leptos`](https://github.com/leptos-rs/leptos)需要）

> *你需要先进行上一步，否则编译目标并不会安装到nightly工具链，将会导致此项目无法编译*
```bash
rustup target add wasm32-unknown-unknown
```

## 构建工具准备

安装[`trunk`](https://trunkrs.dev/)
```bash
cargo install trunk
```

\*安装[`wasm-bindgen-cli`]（这一步不是必须的，因为trunk会自动从github下载这个...但是往往因为*网络问题*失败，不如直接提前使用`cargo`安装好）
```bash
cargo install wasm-bindgen-cli
```

另外，本项目使用了`leptosfmt`对项目进行格式化，如果你不需要可以直接删掉`build.rs`，否则使用以下命令安装
```bash
cargo install leptosfmt
```

## 构建页面

万事俱备


> trunk的使用详见[此处](https://trunkrs.dev/)

如果你准备构建（而非开发）页面，执行以下命令，会再项目目录下的dist文件夹生成相应的文件
```bash
trunk build --release
```

如果你打算*预览*页面，你可以执行以下命令，它会在本地打开一个服务器，此时使用浏览器打开`localhost:12000`即可看见简约（简陋）的页面（详见[配置](./README.md#配置)）
```
trunk serve
```

如果你想直接在服务器打开项目，你可以执行以下命令
```
trunk serve --release
```

# 配置

见[Trunk.toml](./Trunk.toml)，默认配置如下，这个配置只会影响`trunk serve`命令
```toml
[serve]
address = "0.0.0.0"
port = 12001
```

- `address`设置为`127.0.0.1`可以使只有本机可以访问网页，设置为`0.0.0.0`可以使任何设备访问（公开网页）

- `port`为网页的端口号，此处不多赘述

# LICENSE

MIT