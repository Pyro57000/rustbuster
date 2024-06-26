# RUSTBUSTER
Simple little multithreaded directory and subdomain bruteforcer written in rust. I wanted to get some hands on with multi threading and using the reqwest library so I figured this would be a good way to start

# USAGE

-h --help will display usage, 

-t to specify targets, this can be a single target or comma separated targets, just don't use any spaces

--threads to set the number of threads to use

-d for directory wordlist

-s for subdomain wordlist

# BUILDING
git clone this repository and cd into it, then run `cargo build --release` the result will be put in a target/release directory run that binary and you're golden!
I'll also be uploading a release so check the releases page for a pre-built binary.  This binary has been tested on Arch Linux that's fully up to date at the time of building.
