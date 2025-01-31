Rust micro-service using Actix Web and Diesel

To avoid Error in 'cargo build' - Missing sqlite3.lib 

copy sqlite3.lib/dll  TO C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.41.34433\atlmfc\lib\x64
or to later version MSVC dir!

or to : C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Tools\MSVC\14.42.34433\atlmfc\lib\x64

or: 
copy sqlite3.dll and sqlite3.lib to:
  C:\Programas\Rust\cargo\registry\src\index.crates.io-6f17d22bba15001f\windows_x86_64_msvc-0.52.6\lib

