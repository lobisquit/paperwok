all: build render run-server

release: build render run-mongo
	cargo run --release

build: src/dbutils.rs src/main.rs src/model.rs
	cargo build

render: src-elm/main.elm
	elm-make src-elm/main.elm --output website/index.html --yes

run-server:
	(cargo run &)

stop-server:
	killall target/debug/paperwok

run-mongo:
	mongod --dbpath ./data/test_db

stop-mongo:
	killall mongod
