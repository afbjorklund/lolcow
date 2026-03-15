# lolcow

`apptainer run oras://ghcr.io/afbjorklund/lolcow.sif`

`128K	lolcow.sif`

```
 ✔ Scanned for vulnerabilities     [0 vulnerability matches]  
   ├── by severity: 0 critical, 0 high, 0 medium, 0 low, 0 negligible
```

Compared with the regular sif image, based on Ubuntu:

docker://sylabsio/lolcow:latest

[Dockerfile](sylabsio/Dockerfile)

`72M	lolcow_latest.sif`

```
 ✔ Scanned for vulnerabilities     [309 vulnerability matches]  
   ├── by severity: 2 critical, 17 high, 166 medium, 102 low, 22 negligible
```

### Debian

[Dockerfile](debian/Dockerfile)

```
 ✔ Scanned for vulnerabilities     [94 vulnerability matches]  
   ├── by severity: 0 critical, 12 high, 13 medium, 4 low, 53 negligible (12 unknown)
```

### Alpine

[Dockerfile](alpine/Dockerfile)

```
 ✔ Scanned for vulnerabilities     [4 vulnerability matches]  
   ├── by severity: 0 critical, 0 high, 4 medium, 0 low, 0 negligible
```
