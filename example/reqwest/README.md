## reqwest example

`reqwest` depends on the `openssl` to use `https`.But OpenHarmony don't provide it as a pre-dependence, so we need to build it.

`ohos-openssl` is a precompiled binary for OpenHarmony.

## Build

```shell
source ./prepare.sh && ohrs build
```