# Winter-ntex

```sh


if [ ! -d ~/Winter-ntex ]; then cd ~/ && git clone https://github.com/AndyInAi/Winter-ntex.git; fi


# 运行

cd ~/Winter-ntex && chmod +x ./Winter-ntex && ./Winter-ntex

# Starting HTTP server at http://0.0.0.0:8080/

# 测试
curl http://localhost:8080/


# 或者编译源代码后运行

# 安装编译环境

(
	mkdir -p ~/.cargo

	echo '
		[source.crates-io]
		replace-with = "tsinghua"

		[source.tsinghua]
		registry = "http://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
	' > ~/.cargo/config
)

$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# 编译源代码
cd ~/Winter-ntex && cargo build --release

# 运行
cd ~/Winter-ntex && ./target/release/Winter-ntex

# Starting HTTP server at http://0.0.0.0:8080/

# 测试
curl http://localhost:8080/


```
