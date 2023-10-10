docker_build("items", "./items")
k8s_yaml("k8s/items.yaml")
k8s_resource("items", port_forwards=3000)

docker_build("frontend", "./frontend")
k8s_yaml("k8s/frontend.yaml")
k8s_resource("frontend", port_forwards=8000)

local_resource("items-docs", dir="./items", cmd="echo 'hej'", serve_cmd="make -C items docs", deps="./items/openapi.yaml", allow_parallel=True, links="http://localhost:8080")
