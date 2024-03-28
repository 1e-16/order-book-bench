#!/bin/bash

stop_and_remove_container() {
    local container_name="$1"
    # shellcheck disable=SC2155
    local existing_container=$(docker ps -aq -f name="$container_name")

    if [[ -n "$existing_container" ]]; then
        docker stop "$existing_container"
        docker rm "$existing_container"
    fi
}

# shellcheck disable=SC2120
check_scylla_running() {
    local container_name="$1"
    result=$(docker exec "$container_name" cqlsh -e "SHOW VERSION")
    if echo "$result" | grep -q "Scylla"; then
        return 0
    else
        return 1
    fi
}

launch_scylla() {
  local container_name="$1"

  # 检查容器是否存在
  if docker ps -a --format '{{.Names}}' | grep -q "^${container_name}$"; then
    # 容器已存在
    if docker ps --format '{{.Names}}' | grep -q "^${container_name}$"; then
      # 容器正在运行
      echo "${container_name} 已在运行中"
    else
      # 容器处于停止状态，将其启动
      docker start "${container_name}"
      echo "${container_name} 器已启动"
    fi
  else
    # 容器不存在，创建并启动
    docker run -itd --name scylla -p 9042:9042 -d scylladb/scylla
    echo "容器已创建并启动"
  fi

  attempts=0
  while [ $attempts -lt 50 ]; do
      if check_scylla_running "${container_name}"; then
          echo "ScyllaDB 安装完成！"
          break
      fi
      attempts=$((attempts + 1))
      sleep 2
  done
}


launch_scylla "scylla"

container_name="order-book-bench"
stop_and_remove_container $container_name
docker build -t order-book-bench .
# --cpuset-cpus=0,1,2,3
docker run -itd --name "$container_name" -p 8081:8081 --memory=8g order-book-bench

container_name="wrk"
stop_and_remove_container $container_name
docker run -itd --name "$container_name" --network host --memory=8g alpine sh -c \
"apk add wrk && wrk -t4 -c400 -d1m http://192.168.31.134:8081/order"

docker logs -f "$container_name"

stop_and_remove_container $container_name

container_name="order-book-bench"
stop_and_remove_container $container_name


#docker run -itd --name wrk --network host -v C:\Users\none1\Desktop\proj\rust\order-book-bench\wrk:/tmp --memory=8g alpine sh -c "apk add wrk && wrk -t4 -c400 -d1m -s /tmp/post_order.lua http://192.168.31.134:8081/order"


docker run -it --network host alpine sh -c \
"apk add wrk && wrk -t6 -c400 -d1m http://192.168.31.44:8081/order"