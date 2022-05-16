for example in ./examples/*.rs
do 
    FILE="$(basename "$example")"
    cargo run --example "${FILE%.*}" -- $args
done