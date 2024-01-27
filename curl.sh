while true; do
    if inotifywait -r -e modify,create,delete target/release/backend; then
        sleep 1
        curl localhost:8000
    fi
done
