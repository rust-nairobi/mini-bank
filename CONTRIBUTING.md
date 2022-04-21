# Requirements to Contribute

1. Rust newest version installed
2. Basic skills coding in Rust (Advanced skills is most welcome) -Clear readable code is paramount.
3. Postgres Database Ready for use
4. Always create feature branches and create Pull Requests to merge to develop and main branches
5. Node 16 or newer version installed and NPM installed
6. Lots of Patience.

# Configuring mini-bank repo.

1. Clone the repo
2. The only one you will need to edit will be the `DATABASE_URL` and BIND_URL if that port is already in use

# Development Frontend Only

1. Follow all steps up to this point only changing the cargo build command
   to `cargo build --release --features dev-frontend`
2. Then add a .env.local inside the site directory and add the value SITE URL Changing the URL if necessary.
3. You can at this point execute `npm run dev` with the backend executable running and work on the frontend

# Development Full Stack or Backend

1. You have to write clean code and tests for every module or feature you create
2. Ignore the --release argument. Because you are doing development. 
