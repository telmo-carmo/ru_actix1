### Rust micro-service using Actix Web and Diesel

```bash
To avoid Error in 'cargo build' - Missing sqlite3.lib 

copy sqlite3.lib/dll  TO C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.41.34433\atlmfc\lib\x64
or to later version MSVC dir!

or to : C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Tools\MSVC\14.42.34433\atlmfc\lib\x64

or: 
copy sqlite3.dll and sqlite3.lib to:
  C:\Programas\Rust\cargo\registry\src\index.crates.io-6f17d22bba15001f\windows_x86_64_msvc-0.52.6\lib

-- to use a server running HTTPS :
# gen a server x509 cert for localhost
sh ./g1.sh

openssl x509 -in cert.pem -text -noout 

set OPENSSL_LIB_DIR=c:\sdk\vcpkg\installed\x64-windows-static-md\lib
set OPENSSL_INCLUDE_DIR=c:\sdk\vcpkg\installed\x64-windows-static-md\include
set OPENSSL_STATIC=1

cargo build

```
