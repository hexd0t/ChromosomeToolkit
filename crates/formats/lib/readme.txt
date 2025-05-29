These files have been created by creating a def file containing the exports of the dlls using `dumpbin /exports <name>.dll`,
e.g.:

```
EXPORTS
<Export names>
```

And then compiling these into libs using `lib /def:<name>.def /out:<name>.lib`

(Both tools are part of the MSVC toolchain)