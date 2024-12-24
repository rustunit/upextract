test:
    cargo r -- --bundle demoasset/test.unitypackage

test2:
    mkdir tmp_test | true
    cargo r -- --bundle demoasset/test.unitypackage --tmp tmp_test
