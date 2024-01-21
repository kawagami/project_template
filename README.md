# 一個包含前端 & 後台 & 後端模板的專案
* 目的是建立一個本地 & 線上都方便部署的專案
* 複製 .env.example 檔案改名為 .env
* 依照需求修改後，在此專案下指令 docker-compose up -d 即可

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
    * 完成依照 .env 設定的 repo 網址來判斷如何初始化專案
    * 走一個 git repo 管理所有專案的話應該是用不到
