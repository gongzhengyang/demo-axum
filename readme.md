docker compose up -d

cargo install sea-orm-cli

sea-orm-cli generate entity --database-url postgres://demo-user:demo-password@127.0.0.1:5439/demo -o crates/entity/src --lib