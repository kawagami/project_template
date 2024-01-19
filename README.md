# 一個包含前端 & 後台 & 後端模板的專案

## 前端(frontend)
* vue - vite
* 開發時 - docker-compose up -d
* 上線時 - dist 資料夾

## 後台(backstage)
* vue - vite
* 開發時 - docker-compose up -d
* 上線時 - dist 資料夾

## 後端(backend)
* axum
* 開發時 - cargo run || cargo watch -x run
* 上線時 - docker build 過後的 image

## 開發環境的建立 & 線上部署的方式
* 希望都是能直接使用 docker-compose up -d
* 依照 .env 的 COMPOSE_FILE 來控制
    * windowns 的檔案分隔符號是 ;
    * linux & mac 的檔案分隔符號是 :

# 過程紀錄
* 希望 docker-compose up -d 之後能依照 .env 的設定建立起對應的環境
* 所以初版應該要有以下三者
    1. .env 的範例 .env.example
    2. develop 環境的 compose.yaml 檔案 compose.develop.yaml
    3. production 環境的 compose.yaml 檔案 compose.production.yaml
* 撰寫初始化專案的 shell
