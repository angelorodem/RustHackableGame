until ./target/release/ServerCTF; do
    echo "Server crashed with exit code $?.  Respawning.." >&2
    sleep 1
done