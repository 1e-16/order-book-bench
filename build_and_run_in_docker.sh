#!/bin/bash

stop_and_remove_container() {
    local container_name="$1"

    local existing_container=$(docker ps -aq -f name="$container_name")

    if [[ -n "$existing_container" ]]; then
        docker stop "$existing_container"
        docker rm "$existing_container"
    fi
}

container_name="order-book-bench"
stop_and_remove_container $container_name
docker build -t order-book-bench .
docker run -itd --name "$container_name" -p 8081:8081 order-book-bench

container_name="wrk"
stop_and_remove_container $container_name
docker run -itd --name "$container_name" --network host alpine sh -c \
"apk add wrk && wrk -t12 -c400 -d30s http://127.0.0.1:8081/order"

docker logs -f "$container_name"

stop_and_remove_container $container_name

container_name="order-book-bench"
stop_and_remove_container $container_name