d:
	nix develop

s:
	mongosh --port 27017

c:
	cargo build -p rst-client
	./target/debug/rst-client connect ws://localhost:3000/ws

serv:
	cargo run -p rst-server

serv-auth:
	cargo run -p rst-auth-server

j bc:
	cargo build -p rst-client

cmock:
	cargo build -p rst-client
	./target/debug/rst-client \
	register http://127.0.0.1:3345/register \
	--phone 13760283690 \
	--email 1159727122@qq.com \
	-u coosis \
	-p passwd

	./target/debug/rst-client \
	register http://127.0.0.1:3345/register \
	--phone 123456 \
	--email 123456 \
	-u coosis2 \
	-p passwd

send-invite tok:
	./target/debug/rst-client \
	send-request ws://localhost:3000/ws \
	--token {{tok}} \
	--email 123456 \
	--name 'testchat' \
	--description 'a chat for test'

send-msg tok chatid:
	./target/debug/rst-client \
	send-message ws://localhost:3000/ws \
	--token {{tok}} \
	--chat-id {{chatid}} \
	--message 'hello world'

list-invites tok:
	./target/debug/rst-client \
	show-invites ws://localhost:3000/ws \
	--token {{tok}}

list-chat tok:
	./target/debug/rst-client \
	show-chats ws://localhost:3000/ws \
	--token {{tok}}

accept-invite tok chatid:
	./target/debug/rst-client \
	accept-invite ws://localhost:3000/ws \
	-t {{tok}} \
	-i {{chatid}}

up:
	podman container start mongodb
	podman container start valkey

initm:
	podman run \
	--name mongodb \
	-p 27017:27017 \
	-d mongodb/mongodb-community-server:latest
	mongosh --port 27017 mongo_init.js

reset:
	mongosh --port 27017 mongo_init.js

initv:
	podman run \
	--name valkey \
	-p 6379:6379 \
	-p 8080:8080 \
	-v valkey:/data \
	-d valkey/valkey:latest
