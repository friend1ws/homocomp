# homocomp
homopolymer compression program by Rust

## Install

```
git clone https://github.com/friend1ws/homocomp
cd homocomp
cargo build --release
export PATH=${PATH}:`pwd`
```

## Run

```
wget https://api.gdc.cancer.gov/data/254f697d-310d-4d7d-a27b-27fbf767a834 -O GRCh38.d1.vd1.fa.tar.gz
tar xvf GRCh38.d1.vd1.fa.tar.gz
homocomp GRCh38.d1.vd1.fa > GRCh38.d1.vd1.homocomp.fa
```
