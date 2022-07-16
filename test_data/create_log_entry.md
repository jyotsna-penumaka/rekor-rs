How to generate data requiered to upload an artifact on Rekor:

Generate a keypair with:
```
$ openssl ecparam -genkey -name prime256v1 > ec_private.pem
$ openssl ec -in ec_private.pem -pubout > ec_public.pem
```
the above step will provide you with 2 files ec_private.pem (private key) and ec_public.pem (public key).

```
$ openssl dgst -sha256 -sign ec_private.pem -out data.sig data     
Signing file data
Write signature to data.sig
```
Get the sha256 hash of the file
```
$ cat data | sha256sum
c7ead87fa5c82d2b17feece1c2ee1bda8e94788f4b208de5057b3617a42b7413  
```

Get the base64 encoded signature of the artifact:
```
$ cat data.sig | base64 -w 0
MEUCIHWACbBnw+YkJCy2tVQd5i7VH6HgkdVBdP7HRV1IEsDuAiEA19iJNvmkE6We7iZGjHsTkjXV8QhK9iXu0ArUxvJF1N8=% 
```
everything excluding "%" is the signature

Get the base64 encoded public key
```
$ cat ec_public.pem | base64 -w 0
LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUZrd0V3WUhLb1pJemowQ0FRWUlLb1pJemowREFRY0RRZ0FFeEhUTWRSQk80ZThCcGZ3cG5KMlozT2JMRlVrVQpaUVp6WGxtKzdyd1lZKzhSMUZpRWhmS0JZclZraGpHL2lCUjZac2s3Z01iYWZPOG9FM01lUEVvWU93PT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==%
```
everything excluding "%" is the public key