
# yew-app

  

## on Ubuntu:

  
**install rust:** 

    curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh

**update rust:** 

    rustup default nightly
    rustup override set nightly
    rustup update && cargo update

**create new rust app with rocket framework and yew:** 

    cargo install trunk
    rustup target add wasm32-unknown-unknown
    cargo new yew-app
    cd yew-app/

**lunch app**

    trunk serve --open



