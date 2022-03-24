# Rust docker scratch template

Minimal docker image generation with statically linked Rust executable.

Shamelessly based on
[go-docker-scratch-template](https://github.com/jbro/go-docker-scratch-template).

Example Rust program uses [reqwest](https://crates.io/crates/reqwest) to make a
HTTPS request to `google.com` to get a current timestamp that is stored in a
local sqlite database using [sqlite](https://crates.io/crates/sqlite).
[openssl](https://crates.io/crates/openssl) is statically link by enabling the
`vendored` option which make the crate compile and link its own version of
OpenSSL.

## Example

Build image:

```sh
% docker build . --tag=rds
```

Run image:

```sh
% docker run --rm -ti rds
Success: 2022-03-24 22:41:12 CET
```

Examine image:

```sh
% docker save rds | tar x --wildcard '*.tar'
% ls */*.tar | xargs -n1 tar tvf
-rwxr-xr-x 0/0        14456672 2022-03-24 22:40 app
drwxr-xr-x 0/0               0 2022-03-24 22:40 etc/
drwxr-xr-x 0/0               0 2022-03-24 22:40 etc/ssl/
drwxr-xr-x 0/0               0 2022-03-24 22:40 etc/ssl/certs/
-rw-r--r-- 0/0          203223 2022-03-24 22:29 etc/ssl/certs/ca-certificates.crt
