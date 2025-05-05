d:
	nix develop

s:
	mongosh --port 27017

c:
	cargo run -p rst-client -- connect ws://localhost:3000/ws

serv:
	cargo run -p rst-server

cmock:
	cargo run -p rst-client \
	-- register http://127.0.0.1:3345/register \
	--phone 13760283690 \
	--email 1159727122@qq.com \
	-u coosis \
	-p passwd

	cargo run -p rst-client \
	-- register http://127.0.0.1:3345/register \
	--phone 123456 \
	--email 123456 \
	-u coosis2 \
	-p passwd

send-chat tok:
	cargo run -p rst-client -- \
	send-request ws://localhost:3000/ws \
	--token {{tok}} \
	--email 123456 \
	--name 'testchat' \
	--description 'a chat for test'

send-msg tok chatid:
	cargo run -p rst-client -- \
	send-message ws://localhost:3000/ws \
	--token {{tok}} \
	--chat-id {{chatid}} \
	--message 'hello world'

list-chat tok:
	cargo run -p rst-client -- \
	show-invites ws://localhost:3000/ws \
	--token {{tok}}

accept-chat tok chatid:
	cargo run -p rst-client -- \
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

spindb:
	mongosh --port 27017 mongo_init.js

initv:
	podman run \
	--name valkey \
	-p 6379:6379 \
	-p 8080:8080 \
	-v valkey:/data \
	-d valkey/valkey:latest
