# lolcow

`apptainer run oras://ghcr.io/afbjorklund/lolcow.sif`

`128K	lolcow.sif`

```
 ✔ Scanned for vulnerabilities     [0 vulnerability matches]  
   ├── by severity: 0 critical, 0 high, 0 medium, 0 low, 0 negligible
```

Compared with the regular sif image, based on Ubuntu:

`apptainer run docker://docker.io/sylabsio/lolcow`

[Dockerfile](sylabsio/Dockerfile)

`72M	lolcow_latest.sif`

```
 ✔ Scanned for vulnerabilities     [331 vulnerability matches]  
   ├── by severity: 2 critical, 17 high, 171 medium, 113 low, 28 negligible
```

### Ubuntu

[Dockerfile](ubuntu/Dockerfile)

```
 ✔ Scanned for vulnerabilities     [154 vulnerability matches]  
   ├── by severity: 2 critical, 8 high, 89 medium, 44 low, 11 negligible
```

### Debian

[Dockerfile](debian/Dockerfile)

```
 ✔ Scanned for vulnerabilities     [133 vulnerability matches]  
   ├── by severity: 2 critical, 21 high, 46 medium, 10 low, 54 negligible
```

### Alpine

[Dockerfile](alpine/Dockerfile)

```
 ✔ Scanned for vulnerabilities     [4 vulnerability matches]  
   ├── by severity: 0 critical, 1 high, 3 medium, 0 low, 0 negligible
```
