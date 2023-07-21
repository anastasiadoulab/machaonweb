#!/usr/bin/env sh

set -xe

  
openssl req -nodes \
          -newkey rsa:2048 \
          -keyout rsa/node0.key \
          -out rsa/node0.req \
          -sha256 \
          -batch \
          -subj "/CN=node0.localdomain"

openssl rsa \
          -in rsa/node0.key \
          -out rsa/node0.rsa


openssl req -nodes \
          -newkey rsa:2048 \
          -keyout rsa/node1.key \
          -out rsa/node1.req \
          -sha256 \
          -batch \
          -subj "/CN=node1.localdomain"

openssl rsa \
          -in rsa/node1.key \
          -out rsa/node1.rsa

          
for kt in rsa ; do

  openssl x509 -req \
            -in $kt/node0.req \
            -out $kt/node0.cert \
            -CA $kt/machaonlocalca.cert \
            -CAkey $kt/machaonlocalca.key \
            -sha256 \
            -days 2000 \
            -set_serial 456 \
            -extensions v3_node0 -extfile openssl-node0.cnf

  openssl x509 -req \
            -in $kt/node1.req \
            -out $kt/node1.cert \
            -CA $kt/machaonlocalca.cert \
            -CAkey $kt/machaonlocalca.key \
            -sha256 \
            -days 2000 \
            -set_serial 789 \
            -extensions v3_node1 -extfile openssl-node1.cnf

  cat $kt/machaonlocalca.cert > $kt/node0.chain
  cat $kt/node0.cert $kt/machaonlocalca.cert > $kt/node0.fullchain

  cat $kt/machaonlocalca.cert > $kt/node1.chain
  cat $kt/node1.cert $kt/machaonlocalca.cert > $kt/node1.fullchain

  openssl asn1parse -in $kt/machaonlocalca.cert -out $kt/machaonlocalca.der > /dev/null
done

