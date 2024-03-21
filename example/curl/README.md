## curl example

`curl` depends on the `openssl` to use `TLS`.But OpenHarmony don't provide it as a pre-dependence, so we need to build it.

`ohos-openssl` is a precompiled binary for OpenHarmony.

## Build

```shell
source ./prepare.sh && ohrs build
```

## Usage

You need to copy the product to the `libs` folder, which includes the corresponding platform product of `ohos-openssl`.