# reveal-yourself

Using Github Actions to automate cargo build for multiple binaries and create tags.

[1. About](#1-about)  
[2. Intallation](#2-installation)  
[3. What I Did](#3-what-i-did)  
&nbsp; &nbsp; [3-1. Dispatch Custom Event](#3-1-dispatch-custom-event)  
&nbsp; &nbsp; [3-2. Get Current Version](#3-2-get-current-version)  
&nbsp; &nbsp; [3-3. Get Commit Hash](#3-3-get-commit-hash)  
&nbsp; &nbsp; [3-4. Build Multiple Binaries](#3-4-build-multipe-binaries)  
[4. Notes](#4-notes)  
&nbsp; &nbsp; [4-1. PAT Setup](#4-1-pat-setup)  
[5. LICENSE](#5-license)  


## 1. About

Using Github Actions to automate cargo build for multiple binaries and create tags.

## 2. Installation

```
# Download binary
$ wget https://github.com/minagawah/reveal-yourself/releases/download/v{VERSION}-{HASH}/reveal-yourself-linux-x86_64
$ chmod 775 reveal-yourself-linux-x86_64
$ ./reveal-yourself-linux-x86_64

# Or, download the sources, and compile
$ wget https://github.com/minagawah/reveal-yourself/archive/v{VERSION}-{HASH}.tar.gz
tar -xzvf v{VERSION}-{HASH}.tar.gz
cd v{VERSION}-{HASH}
cargo build --release
```

([BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep/tree/31adff6f3c4bfefc9e77df40871f2989443e6827#installation)
has instructions for other targets)


## 3. What I Did

I wanted to use Github Actions to automate several jobs I manually do.  
That are...

- Build for multiple platforms
- Generate a tag based on the package version (defined in `Cargo.toml`)
- Make the binaries available on the release page

### 3-1. Dispatch Custom Event

However, building for multiple binaries requires different VM
should be running in the pipelines,
and it is currently not possible in a single job.  
The solution was found in
[this great post](https://mateuscosta.me/rust-releases-with-github-actions)
which suggests to dispatch an event,
and prepare corresponding jobs triggered by the event dispatched.

So, I am mostly following what the post tells,
and I have the following files under `.github/workflows`:

```
.github/workflows
 ├── main.yml
 └── release.yml
```

In `main.yml`, I am using
[peter-evans/repository-dispatch](https://github.com/peter-evans/repository-dispatch)
to dispatch a custom event, called `tag-created`.
To this event, I can attach any payloads
so that they are shared with jobs defined in `release.yml`.

It would be worth mentioning that
***I had hard time setting up PAT (Personal Access Token)***
which is required when using this event dispatcher,
and I have [a separate note to explain the steps in detail](#4-1-pat-setup).

So, I am dispatching the event with, for instance, the following payload:

```
Ex.
{ "new_version": "v9.9.99-9999999" }
```

whereas for `v9.9.99-9999999` in the above example,
what I actually want is `v{CURRENT_VERSION}-{COMMIT_HASH}`.

### 3-2. Get Current Version

So, I wanted to extract the current version defined in my `Cargo.toml`, and
[toml-cli](https://crates.io/crates/toml-cli)
gives me exactly what I want:

```
$ toml get Cargo.toml package.version | tr -d \"
0.1.3
```

### 3-3. Get Commit Hash

Another thing was a commit hash, and that's easy:

```
git rev-parse --short HEAD
```

### 3-4. Build Multiple Binaries

As `tag-created` event is dispatched,
the jobs in `release.yml` run in parallel manner.
As you can see,
when specifying `use-cross: true` to
[actions-rs/cargo](https://github.com/actions-rs/cargo),
it `cross` instead of `cargo`.
When we actually build,
it is easy as we just specify the target defined in `matrix`:

```yml
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            artifact_name: reveal-yourself
            asset_name: reveal-yourself-linux-x86_64
            target: x86_64-unknown-linux-musl
          - os: macos-latest
            artifact_name: reveal-yourself
            asset_name: reveal-yourself-macos-x86_64
            target: x86_64-apple-darwin

    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Setup Toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
          override: true
      - name: Build
        uses: actions-rs/cargo@v1
        with:
          use-cross: true
          command: build
          args: --release --target ${{ matrix.target }}
```

FYI, here is
[a list of available targets](https://doc.rust-lang.org/beta/rustc/platform-support.html).


## 4. Notes

### 4-1. PAT Setup

When using
[peter-evans/repository-dispatch](https://github.com/peter-evans/repository-dispatch),
as it is mentioned in its description,
even if you already have your GITHUB_TOKEN generated,
it does not allow the event dispatcher to access your repo,
and you must create a new token (e.g. `REPO_ACCESS_TOKEN`).

Without setting PAT, Github Actions throws the following error (when dispatching event):
```
Error: Parameter token or opts.auth is required
```

![Error: Parameter token or opts.auth is required](pat_00.png)

Here are the steps for setting up your PAT:

#### (Step 1) Create a new PAT (Personal Access Tokens)

1. Click your Github profile icon, and you see `Settings`
2. `Settings > Developer Settings > Personal access tokens`
3. Generate a new token with arbitrary chosen name (e.g. `REPO_ACCESS_TOKEN`), and grant `repo` scope.
4. Copy the `secret`

![Creating a new PAT](pat_01.png)

#### (Step 2) Go to your repo's settings

Now, go to your project repository, and go to `Settings > Secrets`.  
Click `New repository secret`.

![Go to your repo](pat_02.png)

#### (Step 3) Set the `secret`

For `secret` you just copied, register to your repo:

![Set the secret](pat_03.png)


## 5. License

Dual-licensed under either of the followings.  
Choose at your option.

- The UNLICENSE ([LICENSE.UNLICENSE](LICENSE.UNLICENSE))
- MIT license ([LICENSE.MIT](LICENSE.MIT))

