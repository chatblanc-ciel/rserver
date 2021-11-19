FROM rust

# rustupのアップデート、target追加
RUN rustup self update \
	&& rustup update \
	&& rustup target add wasm32-unknown-unknown wasm32-wasi

# cargo toolのインストール
RUN cargo install \
	cargo-web \
	wasm-pack \
	wasm-bindgen-cli \
	diesel_cli \
	-f

# apt-get からの各種インストール
RUN apt-get -y update && \
	apt-get -y install \
		nodejs \
		npm \
		apt-utils \
    	git \
    	ca-certificates \
		default-jre \
		default-jdk \
		nodejs npm \
		postgresql \
    && apt-get autoremove -y \
    && apt-get clean -y

# Nodejs, npmのアップデート
RUN npm install -g n \
	&& npm update -g \
	&& n stable \
	&& apt-get purge -y nodejs npm

# data base preset
USER postgres
RUN /etc/init.d/postgresql start && \
    psql --command "CREATE USER root with SUPERUSER CREATEDB CREATEROLE REPLICATION BYPASSRLS LOGIN PASSWORD 'admin';"
USER root

# コンテナ側にworkdirという名前の作業ディレクトリを作り、
# コンテナ起動時に作業ディレクトリに移動する
WORKDIR /workspace
# コンテナ側のリッスンポート番号
EXPOSE 8000