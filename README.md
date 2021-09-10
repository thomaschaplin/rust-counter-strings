<img src="assets/logo.png" alt="logo" width="300"/>

# rust-counter-strings

Counter strings generator written in [rust](https://www.rust-lang.org/) to help aid software testing

## Setup

Make sure you have [rust](https://www.rust-lang.org/) installed on your machine by following the [getting started guide](https://www.rust-lang.org/learn/get-started)

## Instructions

* Clone this repository `git clone git@github.com:thomaschaplin/rust-counter-strings.git`
* Change directory `cd rust-counter-strings`
* Build the application `cargo build`
* Run the application `cargo run <NUMBER>`

### Final Build

* Build the application in release mode `cargo build --release`
* Execute the `rust-counter-strings` binary file found in `target/release/rust-counter-strings`

#### Example output:

./rust-counter-strings 50

```
2*4*6*8*11*14*17*20*23*26*29*32*35*38*41*44*47*50*
```

# Docker Setup

Build
```
docker build --rm -f Dockerfile -t thomaschaplin:rust-counter-strings .
```

Run
```
docker run --rm -it thomaschaplin:rust-counter-strings <NUMBER>
```

---

[Rope](https://www.clipartkey.com/view/imioim_rope-lasso-clipart-rope-black-and-white/) graphic by <a href="https://www.clipartkey.com/upic/322/">Alpenx Nbr</a> from ClipArtKey.
