#!/bin/bash

# Stop all running containers
stop_all_containers() {
    containers=$(docker ps -q)
    if [ -n "$containers" ]; then
        echo "Stopping all running containers..."
        docker stop $containers
    else
        echo "No running containers to stop."
    fi
}

# Remove all exited containers
remove_exited_containers() {
    exited_containers=$(docker ps --filter status=exited -q)
    if [ -n "$exited_containers" ]; then
        echo "Removing exited containers..."
        docker rm -v $exited_containers
    else
        echo "No exited containers to remove."
    fi
}

stop_all_containers
remove_exited_containers