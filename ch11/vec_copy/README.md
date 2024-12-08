# 주의사항

책의 코드는 컴파일러가 최적화활 수 있는 여지가 있는 것 같아서 코드를 살짝 변경했다.

## Debian testing(VMWare)

```nocode
cargo run --release
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/vec_copy`
Naive copy took 0.008495951s
Fast copy took 0.004763198s
Extend took 0.004572952s
```

## Windows 11

```nocode
cargo run --release
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target\release\test2.exe`
Naive copy took 0.0144872s
Fast copy took 0.0104094s
Extend took 0.0102999s
```

성능의 이득은 크지만 책보다는 덜 한 것으로 보인다.

그냥 편하고 빠르고 안전한 extend계열 메서드를 애용하자.
