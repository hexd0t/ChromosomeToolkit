These files have been created by creating a def file containing the exports of the dlls using `dumpbin /exports <name>.dll`,
e.g.:

```
EXPORTS
<Export names>
```

And then compiling these into libs usink `lib /def:<name>.def /out:<name>.lib`