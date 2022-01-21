## Install 

```shell
git clone https://github.com/web3ten0/rust-axum-sqlx-1.git
cd rust-axum-sqlx-1/local
docker-compose up -d
sh scripts/exec_init_db
```

## Migration

- 参照: https://github.com/launchbadge/sqlx/tree/master/sqlx-cli

```shell
cd local/scripts
sh migrate add -r users
sh migrate run
sh migrate revert
```
