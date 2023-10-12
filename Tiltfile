docker_build("items", "./items")
k8s_yaml("k8s/items.yaml")
k8s_resource("items", port_forwards=3000)

docker_build("frontend", "./frontend")
k8s_yaml("k8s/frontend.yaml")
k8s_resource("frontend", port_forwards=8000)

k8s_yaml("k8s/database.yaml")
k8s_resource("database", port_forwards=6379)

local_resource("initialize-database", cmd="kubectl exec deploy/database -- redis-cli SADD tasks:default '0:Read book' '1:Scroll instagram' '2:Eat food' '3:Drink water' '4:Take a nap' '5:1; DROP TABLE users' '6:Watch a movie'", resource_deps=["database"], allow_parallel=True)

local_resource("items-docs", dir="./items", cmd="echo 'hej'", serve_cmd="make -C items docs", deps="./items/openapi.yaml", allow_parallel=True, links="http://localhost:8080")
