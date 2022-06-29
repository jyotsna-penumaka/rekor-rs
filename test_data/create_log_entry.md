How to generate data requiered to upload an artifact on Rekor:

Generate a keypair with:
```
$ ssh-keygen -C test@rekor.dev -t ed25519 -f id_ed25519
```
the following step will provide ypu with 2 files id_ed25519 (private key) and id_ed25519.pub (public key).

```
$ ssh-keygen -Y sign -n file -f id_ed25519 data     
Signing file data
Write signature to data.sig
```
Get the sha256 hash of the file
```
$ cat data | sha256sum
27fcf3e4e65e840060efacd20e272917b9571a29eed63e402fd5e1bfb3ba715d  
```

Get the base64 encoded siganture of the artifact:
```
$ cat data.sig | base64 -w 0
LS0tLS1CRUdJTiBTU0ggU0lHTkFUVVJFLS0tLS0KVTFOSVUwbEhBQUFBQVFBQUFETUFBQUFMYzNOb0xXVmtNalUxTVRrQUFBQWdUUVNxa2dxcmIxK2VNbms2b0c0RlBxNFJHZQpJUVllNjJxL0ZBWVVwcjNxRUFBQUFFWm1sc1pRQUFBQUFBQUFBR2MyaGhOVEV5QUFBQVV3QUFBQXR6YzJndFpXUXlOVFV4Ck9RQUFBRUE5L1JPc0hCQVhMekZ2NWw2OVVyeXFtVTI3Y01wRjRBb1BsMUkzalBQRFBIM2dqUklQYmlEM0c4K2Vud1E1MEEKOFZQWnYrY0YrUEp3ZkdYaGpZbFk0SwotLS0tLUVORCBTU0ggU0lHTkFUVVJFLS0tLS0K%  
```
everything excluding "%" is the signature