test:
    cargo r -- extract -b demoasset/test.unitypackage

test1:
    cargo r -- extract  -b demoasset/test.unitypackage -f

test2:
    mkdir tmp_test | true
    cargo r -- extract -b demoasset/test.unitypackage --tmp tmp_test
