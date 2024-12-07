# 주의

사용한 환경이 달라서인지 책의 내용과는 다른 결과가 나온다.

반복 실행해도 결과가 비슷하다.

## Windows

```nocode
cargo run --release
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target\release\vec_test.exe`
First loop took         0.0076952s
Second loop took        0.0067157s
Third loop took         0.0076833s
```

```nocode
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\vec_test.exe`
First loop took         0.0394821s
Second loop took        0.0401945s
Third loop took         0.0277143s
```

## Linux(VMWare)

```nocode
cargo run --release
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/vec_for_iter`
First loop took         0.002319476s
Second loop took        0.002225898s
Third loop took         0.002335444s
```

```nocode
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/vec_for_iter`
First loop took         0.03675647s
Second loop took        0.03257446s
Third loop took         0.024872124s
```

가상머신에서 실행했던 리눅스보다 실제 머신에서 실행한 윈도가 느리다.

꼬인 부분이 있는 것 같다.
