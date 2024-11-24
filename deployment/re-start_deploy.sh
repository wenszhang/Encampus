sh deployment/clean_deploy.sh

export RUST_LOG="info" 
export LEPTOS_SITE_ADDR="0.0.0.0:8080" 
export LEPTOS_SITE_ROOT="deployment/bin/site"

mkdir deployment/bin
cargo leptos build --release -vv
cp target/release/encampus deployment/bin/

nohup ./deployment/bin/encampus > deployment/deploy.log 2>&1 &
echo $! > deployment/running_encampus_pid