#openssl req -x509 -newkey rsa:4096 -nodes -sha256 -out cert.pem -keyout key.pem -days 3650 \
#  -subj "/C=PT/ST=Lisboa/L=Lisboa/O=NB/OU=DSI/CN=localhost"  -addext "subjectAltName=DNS:localhost"

  # run interactive mode:
  openssl req -x509 -newkey rsa:4096 -nodes -sha256 -out cert.pem -keyout key.pem -days 3650   -addext "subjectAltName=DNS:localhost"

  openssl x509 -in cert.pem -text -noout 
  