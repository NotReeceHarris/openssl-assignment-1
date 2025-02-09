# Installing OpenSSL

First i install winget since i dont have it on my system, second i run the following command to get the openssl package `winget search openssl` which finds 3 packages

```
Name                 Id                         Version Match        Source
---------------------------------------------------------------------------
FireDaemon OpenSSL 3 FireDaemon.OpenSSL         3.4.0   Tag: openssl winget
OpenSSL 3.4.0        ShiningLight.OpenSSL.Dev   3.4.0                winget
OpenSSL Light 3.4.0  ShiningLight.OpenSSL.Light 3.4.0                winget
```

I only care about `FireDaemon OpenSSL 3` so i install that with the following command `winget install --id=FireDaemon.OpenSSL -e` now because my rust isnt playing nice i had to set OPENSSL_DIR with the following command `set OPENSSL_DIR=C:\Program Files\FireDaemon OpenSSL 3` now openssl is all set for rust
