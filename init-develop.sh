#!/bin/bash

. .env
if [ $? != 0 ]
then
    echo ".env 讀取失敗"
    exit
fi

function use_node() {
    docker run --rm -it -v $PWD:/app -w /app -u $(id -u):$(id -g) node:$NODEIMAGETAG $@
}

function create_vue() {

    if [ $1 = "frontend" ]
    then
        # 沒有前端 git repo 網址的話就建立新專案
        if [ "$FRONTENDGITHUBURL" = "" ]
        then
            echo "沒設定前端頁面 repo 使用全新專案"
            # 可使用的參數 https://vitejs.dev/guide/#scaffolding-your-first-vite-project
            use_node npm create vite@latest -y $1 -- --template vue
        else
            echo "使用 git 抓取專案 $FRONTENDGITHUBURL"
            git clone $FRONTENDGITHUBURL $1
        fi
        cd $1
        use_node npm install
    elif [ $1 = "backstage" ]
    then
        # 沒有後台 git repo 網址的話就建立新專案
        if [ "$BACKSTAGEGITHUBURL" = "" ]
        then
            echo "沒設定後台頁面 repo 使用全新專案"
            # 可使用的參數 https://vitejs.dev/guide/#scaffolding-your-first-vite-project
            use_node npm create vite@latest -y $1 -- --template vue
        else
            echo "使用 git 抓取專案 $BACKSTAGEGITHUBURL"
            git clone $BACKSTAGEGITHUBURL $1
        fi
        cd $1
        use_node npm install
    fi

}

function use_rust() {
    docker run --rm -it -v $PWD:/app -w /app -u $(id -u):$(id -g) rust:$RUSTIMAGETAG $@
}

function create_rust() {
    # 沒有後端 git repo 網址的話就建立新專案
    if [ "$BACKENDGITHUBURL" = "" ]
    then
        use_rust cargo new $1
    else
        echo "使用 git 抓取專案 $BACKENDGITHUBURL"
        git clone $BACKENDGITHUBURL $1
    fi
    cd $1
}

function init_project() {
    # $2 初始化專案
    echo $2 初始化專案

    # 使用 docker 建立新的專案
    $1 $2

    cd ..
}

# 移除既有的 frontend backstage backend 資料夾
rm -rf frontend backstage backend

# frontend 初始化專案
init_project create_vue frontend

# backstage 初始化專案
init_project create_vue backstage

# # backend 初始化專案
init_project create_rust backend
