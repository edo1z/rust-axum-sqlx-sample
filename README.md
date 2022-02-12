# rust-axum-sqlx-sample

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

## Swagger
### swagger-uiのURL
- http://localhost:8001

### OpenAPIのBundle方法

- [swagger-cli](https://github.com/APIDevTools/swagger-cli)を使ってbundleする。

```shell
> npm install -g @apidevtools/swagger-cli
> swagger-cli bundle -o openapi.yaml -t yaml openapi/base.yaml
```

#### bundle script

- 上記のswagger-cliコマンドを書いた。下記の`openapi/bundle`を実行したら、openapi.yamlが最新状態になる。

```shell
> sh openapi/bundle
```
