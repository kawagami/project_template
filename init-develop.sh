#!/bin/bash

NODEIMAGETAG="18-slim"
RUSTIMAGETAG="1.74.0-slim-bookworm"

function use_node() {
    docker run --rm -it -v $PWD:/app -w /app -u $(id -u):$(id -g) node:$NODEIMAGETAG $@
}

function create_vue() {
    # 使用 docker 建立新的 vue 專案
    # 可使用的參數 https://vitejs.dev/guide/#scaffolding-your-first-vite-project
    use_node npm create vite@latest -y $1 -- --template vue

    cd $1

    use_node npm install
}

function use_rust() {
    docker run --rm -it -v $PWD:/app -w /app -u $(id -u):$(id -g) rust:$RUSTIMAGETAG $@
}

function create_rust() {
    use_rust cargo new $1

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
rm -rf  frontend backstage backend

# frontend 初始化專案
init_project create_vue frontend

# backstage 初始化專案
init_project create_vue backstage

# # backend 初始化專案
init_project create_rust backend
