# sgf2json
Program to convert a bunch of SGF files into a single JSON file for better machine readability. Part of the https://github.com/q-Sci/tsmgo repository, the output JSON can be used as the puzzle source for the application. The program was specifically designed with go problems (tsumego) in mind.

The input SGF files should look like the following example:
```
(;GM[1]FF[4]SZ[19]
CP[Xuanxuan Qijing Problems]
BR[10k]WR[10k]

AB[il][kl][kj][hj][ii]
AW[jk]
PL[W]

;W[jj];B[ki];W[ik];B[hl];W[ji];B[jh];W[ih];B[kk];W[hi];B[ij];W[hk])
```
